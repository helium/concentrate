use super::{msg_send, print_at_level};
use crate::{cfg, error::AppResult};
use loragw;
use messages::*;
use protobuf::parse_from_bytes;
use std::{
    convert::{TryFrom, TryInto},
    io::ErrorKind,
    net::{SocketAddr, UdpSocket},
    time::Duration,
};

pub fn serve(
    cfg: Option<&str>,
    polling_interval: u64,
    print_level: u8,
    req_port: u16,
    resp_port: u16,
) -> AppResult {
    let (socket, resp_addr) = {
        let resp_addr = SocketAddr::from(([127, 0, 0, 1], resp_port));
        let req_addr = SocketAddr::from(([127, 0, 0, 1], req_port));
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

    loop {
        while let Some(packets) = concentrator.receive()? {
            for pkt in packets {
                print_at_level(print_level, &pkt);
                if let loragw::RxPacket::LoRa(pkt) = pkt {
                    let resp = Resp {
                        id: 0,
                        kind: Some(Resp_oneof_kind::rx_packet(pkt.into())),
                        ..Default::default()
                    };
                    msg_send(resp, &socket, resp_addr)?;
                }
            }
        }

        match socket.recv(&mut req_buf) {
            Ok(sz) => {
                let resp = match parse_from_bytes::<Req>(&req_buf[..sz]) {
                    Ok(req) => match req {
                        // Valid TX request
                        Req {
                            id,
                            kind: Some(Req_oneof_kind::tx(req)),
                            ..
                        } => match concentrator.transmit(loragw::TxPacket::LoRa(req.into())) {
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
                        },
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