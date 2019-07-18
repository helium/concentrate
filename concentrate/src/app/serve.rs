use super::{msg_send, print_at_level};
use crate::{cfg, error::AppResult};
use loragw;
use messages::*;
use protobuf::parse_from_bytes;
use std::{
    convert::{TryFrom, TryInto},
    io::ErrorKind,
    net::{IpAddr, SocketAddr, UdpSocket},
    time::Duration,
};

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
        println!("req port: {}", req_addr);
        println!("resp port: {}", resp_addr);
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
                    println!("received {:?}", pkt);
                    let resp = RadioResp {
                        id: 0,
                        kind: Some(RadioResp_oneof_kind::rx_packet(pkt.into())),
                        ..Default::default()
                    };
                    msg_send(resp, &socket, resp_addr)?;
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
                            println!("transmitting {:?}", pkt);
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
