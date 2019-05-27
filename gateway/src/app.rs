use crate::error;
use log;
use loragw;
use messages;
use protobuf::Message;
use std::{
    io::ErrorKind,
    net::{SocketAddr, UdpSocket},
    time,
};

pub fn go(
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
    config(&concentrator)?;
    concentrator.start()?;

    let mut tx_req_buf = [0; 1024];
    let mut rx_buf = Vec::new();

    loop {
        while let Some(packets) = concentrator.receive()? {
            for pkt in packets {
                if print_level > 1 {
                    println!("{:#?}\n", pkt);
                } else if print_level == 1 {
                    println!("{:?}\n", pkt);
                }
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
                let tx_pkt = parse_tx_req(&tx_req_buf)?;
                concentrator.transmit(tx_pkt)?;
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => (),
            Err(e) => return Err(e.into()),
        }
    }
}

fn parse_tx_req(_buf: &[u8]) -> error::Result<loragw::TxPacket> {
    unimplemented!()
}

fn config(concentrator: &loragw::Concentrator) -> error::Result {
    let board_conf = loragw::BoardConf {
        lorawan_public: false,
        clksrc: loragw::Radio::R1,
    };
    concentrator.config_board(&board_conf)?;

    concentrator.config_rx_rf(
        0,
        &loragw::RxRFConf {
            enable: true,
            freq: 911_500_000,
            rssi_offset: -162.0,
            type_: loragw::RadioType::SX1257,
            tx_enable: true,
            tx_notch_freq: 126_000,
        },
    )?;

    concentrator.config_rx_rf(
        1,
        &loragw::RxRFConf {
            enable: true,
            freq: 903_500_000,
            rssi_offset: -162.0,
            type_: loragw::RadioType::SX1257,
            tx_enable: false,
            tx_notch_freq: 0,
        },
    )?;

    // chan_multiSF_0
    concentrator.config_channel(
        0,
        &loragw::ChannelConf::Multirate {
            radio: loragw::Radio::R0,
            freq: -400_000,
        },
    )?;

    // chan_multiSF_1
    concentrator.config_channel(
        1,
        &loragw::ChannelConf::Multirate {
            radio: loragw::Radio::R0,
            freq: -200_000,
        },
    )?;

    // chan_multiSF_2
    concentrator.config_channel(
        2,
        &loragw::ChannelConf::Multirate {
            radio: loragw::Radio::R0,
            freq: 0,
        },
    )?;

    // chan_multiSF_3
    concentrator.config_channel(
        3,
        &loragw::ChannelConf::Multirate {
            radio: loragw::Radio::R0,
            freq: 200_000,
        },
    )?;

    // "chan_multiSF_4"
    concentrator.config_channel(
        4,
        &loragw::ChannelConf::Multirate {
            radio: loragw::Radio::R1,
            freq: -400_000,
        },
    )?;

    // chan_multiSF_5
    concentrator.config_channel(
        5,
        &loragw::ChannelConf::Multirate {
            radio: loragw::Radio::R1,
            freq: -200_000,
        },
    )?;

    // chan_multiSF_6
    concentrator.config_channel(
        6,
        &loragw::ChannelConf::Multirate {
            radio: loragw::Radio::R1,
            freq: 0,
        },
    )?;

    // chan_multiSF_7
    concentrator.config_channel(
        7,
        &loragw::ChannelConf::Multirate {
            radio: loragw::Radio::R1,
            freq: 200_000,
        },
    )?;

    // LoRa STD
    concentrator.config_channel(
        8,
        &loragw::ChannelConf::Fixed {
            radio: loragw::Radio::R0,
            freq: 300_000,
            bandwidth: loragw::Bandwidth::BW500kHz,
            spreading: loragw::Spreading::SF8,
        },
    )?;

    // [G]FSK
    concentrator.config_channel(9, &loragw::ChannelConf::Disable)?;

    Ok(())
}
