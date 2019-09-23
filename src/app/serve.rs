use super::{msg_send, print_at_level};
use crate::{cfg, cmdline, error::AppResult};
use loragw;
use messages::*;
use protobuf::parse_from_bytes;
use std::{
    convert::{TryFrom, TryInto},
    fs,
    io::ErrorKind,
    net::UdpSocket,
    path::PathBuf,
    time::Duration,
};

pub fn serve(args: cmdline::Serve) -> AppResult {
    let socket = {
        assert_ne!(args.listen_addr_in, args.publish_addr_out);
        log::debug!("listen addr: {}", args.listen_addr_in);
        log::debug!("publish addr: {}", args.publish_addr_out);
        UdpSocket::bind(args.listen_addr_in)?
    };

    socket.set_read_timeout(Some(Duration::from_millis(args.interval)))?;
    let mut req_buf = [0; 1024];

    let mut concentrator = loragw::Concentrator::open()?;
    config(&mut concentrator, args.cfg_file)?;
    concentrator.start()?;

    loop {
        while let Some(packets) = concentrator.receive()? {
            for pkt in packets {
                print_at_level(args.print_level, &pkt);
                if let loragw::RxPacket::LoRa(pkt) = pkt {
                    log::debug!("received {:?}", pkt);
                    let resp = RadioResp {
                        id: 0,
                        kind: Some(RadioResp_oneof_kind::rx_packet(pkt.into())),
                        ..Default::default()
                    };
                    msg_send(resp, &socket, args.publish_addr_out)?;
                }
            }
        }

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
                            log::debug!("transmitting {:?}", pkt);
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
                            log::error!("request {} empty", id);
                            RadioResp {
                                id,
                                kind: None,
                                ..Default::default()
                            }
                        }
                    },
                    Err(e) => {
                        log::error!("parse Req error {:?} from {:x?}", e, &req_buf[..sz]);
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

fn config(concentrator: &mut loragw::Concentrator, cfg: Option<PathBuf>) -> AppResult {
    let cfg = match cfg {
        Some(path) => {
            let cfg_str = fs::read_to_string(path)?;
            cfg::Config::from_str(&cfg_str)?
        }
        None => cfg::Config::from_str_or_default(None)?,
    };

    log::debug!("configuring concentrator with {:?}", cfg);

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
