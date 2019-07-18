use super::{msg_send, print_at_level};
use crate::{cfg::Config, cmdline, error::AppResult};
use loragw;
use messages::*;
use protobuf::{parse_from_bytes, well_known_types};
use std::{
    convert::{TryFrom, TryInto},
    error::Error,
    fs,
    io::{BufReader, ErrorKind, Read},
    net::UdpSocket,
    sync::mpsc,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub fn serve(args: cmdline::Serve) -> AppResult {
    let socket = {
        assert_ne!(args.listen_addr_in, args.publish_addr_out);
        debug!("listen addr: {}", args.listen_addr_in);
        debug!("publish addr: {}", args.publish_addr_out);
        UdpSocket::bind(args.listen_addr_in)?
    };

    socket.set_read_timeout(Some(Duration::from_millis(args.interval)))?;
    let mut req_buf = [0; 1024];

    let cfg = match args.cfg_file {
        None => Config::default(),
        Some(path) => Config::from_str(fs::read_to_string(path)?)?,
    };

    let mut concentrator = loragw::Concentrator::open()?;
    config_concentrator(&mut concentrator, &cfg)?;
    concentrator.start()?;

    let (sender, receiver) = mpsc::channel();
    let (mut gps, tty) = loragw::GPS::open(cfg.board.gps, 9600, Some(&concentrator))?;
    thread::spawn(move || gps_deframer(tty, sender));

    loop {
        // Take available radio packets from the concentrator.
        while let Some(packets) = concentrator.receive()? {
            for pkt in packets {
                print_at_level(args.print_level, &pkt);
                if let loragw::RxPacket::LoRa(pkt) = pkt {
                    debug!("received {:?}", pkt);
                    let (_timestamp_is_gps, _timestamp) =
                        if let Ok(t) = gps.systemtime_from_timestamp(pkt.timestamp) {
                            (true, t)
                        } else {
                            (false, SystemTime::now())
                        };
                    let resp = RadioResp {
                        id: 0,
                        kind: Some(RadioResp_oneof_kind::rx_packet(pkt.into())),
                        ..Default::default()
                    };
                    msg_send(resp, &socket, args.publish_addr_out)?;
                }
            }
        }

        // Take available GPS frames from the deframer thread.
        while let Ok(frame) = receiver.try_recv() {
            let _ = gps.parse(frame);
        }

        // Receive and process client requests.
        match socket.recv(&mut req_buf) {
            Ok(sz) => {
                let resp = match parse_from_bytes::<RadioReq>(&req_buf[..sz]) {
                    Ok(req) => match req {
                        // Valid TX request
                        RadioReq {
                            id,
                            kind: Some(RadioReq_oneof_kind::tx(req)),
                            ..
                        } => {
                            let pkt = req.into();
                            debug!("transmitting {:?}", pkt);
                            match concentrator.transmit(loragw::TxPacket::LoRa(pkt)) {
                                Ok(()) => RadioResp {
                                    id,
                                    kind: Some(RadioResp_oneof_kind::tx(RadioTxResp {
                                        success: true,
                                        ..Default::default()
                                    })),
                                    ..Default::default()
                                },
                                Err(_) => RadioResp {
                                    id,
                                    kind: Some(RadioResp_oneof_kind::tx(RadioTxResp {
                                        success: false,
                                        ..Default::default()
                                    })),
                                    ..Default::default()
                                },
                            }
                        }
                        // Invalid request
                        RadioReq { id, kind: None, .. } => {
                            error!("request {} empty", id);
                            RadioResp {
                                id,
                                kind: None,
                                ..Default::default()
                            }
                        }
                    },
                    Err(e) => {
                        error!("parse Req error {:?} from {:x?}", e, &req_buf[..sz]);
                        RadioResp {
                            id: 0,
                            kind: Some(RadioResp_oneof_kind::parse_err(Vec::from(&req_buf[..sz]))),
                            ..Default::default()
                        }
                    }
                };
                msg_send(resp, &socket, args.publish_addr_out)?;
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => (),
            Err(e) => return Err(e.into()),
        }
    }
}

fn config_concentrator(concentrator: &mut loragw::Concentrator, cfg: &Config) -> AppResult {
    // TODO: remove the need for the following `.clone()`s. It's not a
    // performance issue here, as this function is called only once,
    // but it's amateurish.
    let board_cfg = cfg.board.clone();
    concentrator.config_board(&board_cfg.try_into()?)?;

    if let Some(radios) = cfg.radios.clone() {
        for c in radios {
            concentrator.config_rx_rf(&loragw::RxRFConf::try_from(c)?)?;
        }
    }

    if let Some(multirate_channels) = cfg.multirate_channels.clone() {
        for (i, c) in multirate_channels.iter().enumerate() {
            concentrator.config_channel(i as u8, &loragw::ChannelConf::try_from(c)?)?;
        }
    }

    if let Some(gains) = cfg.tx_gains.clone() {
        let gains: Vec<loragw::TxGain> = gains
            .iter()
            .map(|g| loragw::TxGain::from(g.clone()))
            .collect();
        concentrator.config_tx_gain(gains.as_slice())?
    }

    Ok(())
}

fn gps_deframer(tty: fs::File, sender: mpsc::Sender<loragw::Frame>) {
    let mut deframer = loragw::Deframer::new();
    let rdr = BufReader::new(tty);
    for byte in rdr.bytes() {
        match byte {
            Err(e) => error!("error reading GPS TTY: {}", e.description()),
            Ok(byte) => {
                if let Some(msg) = deframer.push(byte) {
                    sender
                        .send(msg)
                        .unwrap_or_else(|e: mpsc::SendError<loragw::Frame>| {
                            error!("send error: {}", e)
                        });
                }
            }
        }
    }
}

fn prototimestamp_from_systemtime(sys_time: SystemTime) -> well_known_types::Timestamp {
    let unix_dur = sys_time
        .duration_since(UNIX_EPOCH)
        .expect("cannot convert a timestamp before the Unix epoch to a duration");
    well_known_types::Timestamp {
        seconds: unix_dur.as_secs() as i64,
        nanos: unix_dur.subsec_nanos() as i32,
        ..Default::default()
    }
}
