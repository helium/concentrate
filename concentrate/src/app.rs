use crate::{cfg, error};
use log;
use loragw;
use messages;
use protobuf::{parse_from_bytes, Message};
use std::{
    convert::{TryFrom, TryInto},
    error::Error,
    fmt,
    io::ErrorKind,
    net::{SocketAddr, UdpSocket},
    time,
};

fn print_pkt<T: fmt::Debug>(print_level: u8, pkt: &T) {
    if print_level > 1 {
        println!("{:#?}\n", pkt);
    } else if print_level == 1 {
        println!("{:?}\n", pkt);
    }
}

pub fn serve(
    cfg: Option<&str>,
    polling_interval: u64,
    print_level: u8,
    listen_port: u16,
    publish_port: u16,
) -> error::Result {
    let listen_addr = SocketAddr::from(([127, 0, 0, 1], listen_port));
    let publish_addr = SocketAddr::from(([127, 0, 0, 1], publish_port));
    assert_ne!(listen_addr, publish_addr);
    log::debug!("listening for TX packets on {}", listen_addr);
    log::debug!("publishing received packets to {}", publish_addr);
    let socket = UdpSocket::bind(listen_addr)?;
    socket.set_read_timeout(Some(time::Duration::from_millis(polling_interval)))?;

    let concentrator = loragw::Concentrator::open()?;
    config(&concentrator, cfg)?;
    concentrator.start()?;

    let mut tx_req_buf = [0; 1024];
    let mut rx_buf = Vec::new();

    loop {
        while let Some(packets) = concentrator.receive()? {
            for pkt in packets {
                print_pkt(print_level, &pkt);
                if let loragw::RxPacket::LoRa(pkt) = pkt {
                    let proto_pkt: messages::RxPacket = pkt.into();
                    proto_pkt
                        .write_to_vec(&mut rx_buf)
                        .expect("error serializing RxPacket");
                    log::debug!("encoded RxPacket into {} bytes", rx_buf.len());
                    socket.send_to(&rx_buf, publish_addr)?;
                    rx_buf.truncate(0);
                }
            }
        }
        match socket.recv(&mut tx_req_buf) {
            Ok(sz) => {
                log::debug!("Read {} bytes {:?}", sz, &tx_req_buf[..sz]);
                match parse_from_bytes::<messages::TxPacket>(&tx_req_buf[..sz]) {
                    Ok(tx_pkt) => {
                        log::debug!("received tx req {:?}", tx_pkt);
                        let tx_pkt = loragw::TxPacket::LoRa(tx_pkt.into());
                        concentrator.transmit(tx_pkt).unwrap_or_else(|e| {
                            log::error!("transmit failed with '{}'", e.description())
                        });
                    }
                    Err(e) => log::error!("{:?}", e),
                }
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => (),
            Err(e) => return Err(e.into()),
        }
    }
}

pub fn listen(print_level: u8, publish_port: u16) -> error::Result {
    let publish_addr = SocketAddr::from(([127, 0, 0, 1], publish_port));
    log::debug!("listening for published packets on {}", publish_addr);
    let socket = UdpSocket::bind(publish_addr)?;

    let mut udp_read_buf = [0; 1024];

    loop {
        let (sz, src) = socket.recv_from(&mut udp_read_buf)?;
        log::debug!("read {} bytes from {}", sz, src);
        match parse_from_bytes::<messages::RxPacket>(&udp_read_buf[..sz]) {
            Ok(rx_pkt) => print_pkt(print_level, &rx_pkt),
            Err(e) => log::error!("{:?}", e),
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn send(
    listen_port: u16,
    freq: u32,
    radio: u8,
    power: i8,
    spreading: u8,
    coderate: u8,
    bandwidth: u32,
    payload: Option<String>,
) -> error::Result {
    // unimplemented!()
    let tx_pkt = messages::TxPacket {
        freq,
        radio: match radio {
            0 => messages::Radio::R0,
            1 => messages::Radio::R1,
            e => return Err(error::Error::Generic(format!("{} is not a valid radio", e))),
        },
        power: i32::from(power),
        bandwidth: match bandwidth {
            7800 => messages::Bandwidth::BW7_8kHz,
            15600 => messages::Bandwidth::BW15_6kHz,
            31200 => messages::Bandwidth::BW31_2kHz,
            62500 => messages::Bandwidth::BW62_5kHz,
            125_000 => messages::Bandwidth::BW125kHz,
            250_000 => messages::Bandwidth::BW250kHz,
            500_000 => messages::Bandwidth::BW500kHz,
            e => {
                return Err(error::Error::Generic(format!(
                    "{} is not a valid bandwidth",
                    e
                )));
            }
        },
        spreading: match spreading {
            7 => messages::Spreading::SF7,
            8 => messages::Spreading::SF8,
            9 => messages::Spreading::SF9,
            10 => messages::Spreading::SF10,
            11 => messages::Spreading::SF11,
            12 => messages::Spreading::SF12,
            e => {
                return Err(error::Error::Generic(format!(
                    "{} is not a valid spreading factor",
                    e
                )));
            }
        },
        coderate: match coderate {
            5 => messages::Coderate::CR4_5,
            6 => messages::Coderate::CR4_6,
            7 => messages::Coderate::CR4_7,
            8 => messages::Coderate::CR4_8,
            e => {
                return Err(error::Error::Generic(format!(
                    "4/{} is not a valid coderate",
                    e
                )));
            }
        },
        invert_polarity: false,
        omit_crc: false,
        implicit_header: false,
        payload: payload.unwrap_or_default().into_bytes(),
        ..Default::default()
    };
    log::debug!("requesting to transmit {:#?}", tx_pkt);
    let socket = UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], 0)))?;
    let listen_addr = SocketAddr::from(([127, 0, 0, 1], listen_port));
    let mut tx_buf = Vec::new();
    tx_pkt
        .write_to_vec(&mut tx_buf)
        .expect("error serializing TxPacket");
    log::debug!("encoded TxPacket into {} bytes", tx_buf.len());
    socket.send_to(&tx_buf, listen_addr)?;
    Ok(())
}

fn config(concentrator: &loragw::Concentrator, cfg: Option<&str>) -> error::Result {
    let cfg = cfg::Config::from_str_or_default(cfg)?;
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

    Ok(())
}
