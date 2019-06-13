use crate::{cfg, error::AppResult};
use loragw;
use protobuf::{parse_from_bytes, Message};
use std::{
    convert::{TryFrom, TryInto},
    error::Error,
    io::ErrorKind,
    net::{SocketAddr, UdpSocket},
    time,
};

pub fn serve(
    cfg: Option<&str>,
    polling_interval: u64,
    print_level: u8,
    listen_port: u16,
    publish_port: u16,
) -> AppResult {
    let (socket, publish_addr) = {
        let publish_addr = SocketAddr::from(([127, 0, 0, 1], publish_port));
        let listen_addr = SocketAddr::from(([127, 0, 0, 1], listen_port));
        assert_ne!(listen_addr, publish_addr);
        debug!("listening for TX packets on {}", listen_addr);
        debug!("publishing received packets to {}", publish_addr);
        (UdpSocket::bind(listen_addr)?, publish_addr)
    };

    socket.set_read_timeout(Some(time::Duration::from_millis(polling_interval)))?;
    let mut tx_req_buf = [0; 1024];
    let mut rx_buf = Vec::new();

    let mut concentrator = loragw::Concentrator::open()?;
    config(&mut concentrator, cfg)?;
    concentrator.start()?;

    loop {
        while let Some(packets) = concentrator.receive()? {
            for pkt in packets {
                super::print_at_level(print_level, &pkt);
                if let loragw::RxPacket::LoRa(pkt) = pkt {
                    let proto_pkt: messages::RxPacket = pkt.into();
                    proto_pkt
                        .write_to_vec(&mut rx_buf)
                        .expect("error serializing RxPacket");
                    debug!("encoded RxPacket into {} bytes", rx_buf.len());
                    socket.send_to(&rx_buf, publish_addr)?;
                    rx_buf.truncate(0);
                }
            }
        }

        match socket.recv(&mut tx_req_buf) {
            Ok(sz) => {
                debug!("Read {} bytes {:?}", sz, &tx_req_buf[..sz]);
                match parse_from_bytes::<messages::TxPacket>(&tx_req_buf[..sz]) {
                    Ok(tx_pkt) => {
                        debug!("received tx req {:?}", tx_pkt);
                        let tx_pkt = loragw::TxPacket::LoRa(tx_pkt.into());
                        concentrator.transmit(tx_pkt).unwrap_or_else(|e| {
                            error!("transmit failed with '{}'", e.description())
                        });
                    }
                    Err(e) => error!("{:?}", e),
                }
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
