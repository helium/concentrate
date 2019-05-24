use loragw;
use std::{thread, time};

pub fn go(polling_interval: u64, print_level: u8) -> Result<(), loragw::Error> {
    let concentrator = loragw::Concentrator::open()?;

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

    concentrator.start()?;

    // let mut counter: u32 = 0;
    // loop {
    //     use loragw::*;
    //     concentrator.transmit(TxPacket::LoRa(TxPacketLoRa {
    //         freq: 911_000_000,
    //         mode: TxMode::Immediate,
    //         radio: Radio::R0,
    //         power: 10,
    //         bandwidth: Bandwidth::BW125kHz,
    //         spreading: Spreading::SF10,
    //         coderate: Coderate::Cr4_5,
    //         invert_polarity: true,
    //         preamble: None,
    //         omit_crc: false,
    //         implicit_header: false,
    //         payload: {
    //             let mut wtr = Vec::new();
    //             wtr.write_u32::<BigEndian>(counter).unwrap();
    //             wtr
    //         },
    //     }))?;
    //     thread::sleep(time::Duration::from_millis(300));
    //     counter += 1;
    // }

    loop {
        while let Some(packets) = concentrator.receive()? {
            for pkt in packets {
                if print_level > 1 {
                    println!("{:#?}\n", pkt);
                } else if print_level == 1 {
                    println!("{:?}\n", pkt);
                }
            }
        }
        thread::sleep(time::Duration::from_millis(polling_interval));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_proto_round_trip() {
        use crate::gateway::*;
        use protobuf::{parse_from_bytes, Message};
        let mut buf = Vec::new();
        let pkt0 = RxLoraPacket::new();
        pkt0.write_to_vec(&mut buf).unwrap();
        let pkt1: RxLoraPacket = parse_from_bytes(&buf).unwrap();
        assert_eq!(pkt0, pkt1);
    }
}
