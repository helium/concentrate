use super::{msg_send, print_at_level};
use crate::{cfg, error::AppResult};
use loragw;
use messages::*;
use protobuf::{parse_from_bytes, well_known_types, SingularPtrField};
use std::{
    convert::{TryFrom, TryInto},
    error::Error,
    fs::File,
    io::{BufReader, ErrorKind, Read},
    net::{IpAddr, SocketAddr, UdpSocket},
    sync::mpsc,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

fn gps_deframer(tty: File, sender: mpsc::Sender<loragw::Frame>) {
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

pub fn serve(
    cfg: Option<&str>,
    polling_interval: u64,
    print_level: u8,
    req_port: u16,
    resp_port: u16,
    ip: Option<IpAddr>,
) -> AppResult {
    let (socket, resp_addr) = {
        let resp_addr;
        let req_addr;

        if let Some(remote_ip) = ip {
            resp_addr = SocketAddr::from((remote_ip, resp_port));
            req_addr = SocketAddr::from(([0, 0, 0, 0], req_port));
        } else {
            resp_addr = SocketAddr::from(([127, 0, 0, 1], resp_port));
            req_addr = SocketAddr::from(([127, 0, 0, 1], req_port));
        }

        assert_ne!(req_addr, resp_addr);
        debug!("req port: {}", req_addr);
        debug!("resp port: {}", resp_addr);
        (UdpSocket::bind(req_addr)?, resp_addr)
    };

    socket.set_read_timeout(Some(Duration::from_millis(polling_interval)))?;
    let mut req_buf = [0; 1024];

    let mut concentrator = loragw::Concentrator::open()?;
    config(&mut concentrator, cfg)?;
    concentrator.start()?;

    let (sender, receiver) = mpsc::channel();
    let (mut gps, tty) = loragw::GPS::open("/dev/ttyS0", 9600, Some(&concentrator))?;
    thread::spawn(move || gps_deframer(tty, sender));

    loop {
        // Take available radio packets from the concentrator.
        while let Some(packets) = concentrator.receive()? {
            for pkt in packets {
                print_at_level(print_level, &pkt);
                if let loragw::RxPacket::LoRa(pkt) = pkt {
                    debug!("received {:?}", pkt);
                    let (timestamp_is_gps, timestamp) =
                        if let Ok(t) = gps.systemtime_from_timestamp(pkt.timestamp) {
                            (true, t)
                        } else {
                            (false, SystemTime::now())
                        };
                    let mut proto_pkt: messages::RxPacket = pkt.into();
                    proto_pkt.timestamp_is_gps = timestamp_is_gps;
                    proto_pkt.timestamp =
                        SingularPtrField::some(prototimestamp_from_systemtime(timestamp));
                    let resp = Resp {
                        id: 0,
                        kind: Some(Resp_oneof_kind::rx_packet(proto_pkt)),
                        ..Default::default()
                    };
                    msg_send(resp, &socket, resp_addr)?;
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
                let resp = match parse_from_bytes::<Req>(&req_buf[..sz]) {
                    Ok(req) => match req {
                        // Valid TX request
                        Req {
                            id,
                            kind: Some(Req_oneof_kind::tx(req)),
                            ..
                        } => {
                            let pkt = req.into();
                            debug!("transmitting {:?}", pkt);
                            match concentrator.transmit(loragw::TxPacket::LoRa(pkt)) {
                                Ok(()) => Resp {
                                    id,
                                    kind: Some(Resp_oneof_kind::tx(TxResp {
                                        success: true,
                                        ..Default::default()
                                    })),
                                    ..Default::default()
                                },
                                Err(_) => Resp {
                                    id,
                                    kind: Some(Resp_oneof_kind::tx(TxResp {
                                        success: false,
                                        ..Default::default()
                                    })),
                                    ..Default::default()
                                },
                            }
                        }
                        // Invalid request
                        Req { id, kind: None, .. } => {
                            error!("request {} empty", id);
                            Resp {
                                id,
                                kind: None,
                                ..Default::default()
                            }
                        }
                    },
                    Err(e) => {
                        error!("parse Req error {:?} from {:x?}", e, &req_buf[..sz]);
                        Resp {
                            id: 0,
                            kind: Some(Resp_oneof_kind::parse_err(Vec::from(&req_buf[..sz]))),
                            ..Default::default()
                        }
                    }
                };
                msg_send(resp, &socket, resp_addr)?;
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => (),
            Err(e) => return Err(e.into()),
        }
    }
}

fn config(concentrator: &mut loragw::Concentrator, cfg: Option<&str>) -> AppResult {
    let cfg = cfg::Config::from_str_or_default(cfg)?;
    debug!("configuring concentrator with {:?}", cfg);

    concentrator.config_board(&cfg.board.try_into()?)?;

    if let Some(radios) = cfg.radios {
        for c in radios {
            concentrator.config_rx_rf(&loragw::RxRFConf::try_from(c)?)?;
        }
    }

    if let Some(multirate_channels) = cfg.multirate_channels {
        for (i, c) in multirate_channels.iter().enumerate() {
            concentrator.config_channel(i as u8, &loragw::ChannelConf::try_from(c)?)?;
        }
    }

    if let Some(gains) = cfg.tx_gains {
        let gains: Vec<loragw::TxGain> = gains
            .iter()
            .map(|g| loragw::TxGain::from(g.clone()))
            .collect();
        concentrator.config_tx_gain(gains.as_slice())?
    }

    Ok(())
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
