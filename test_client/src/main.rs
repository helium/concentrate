use env_logger;
use loragw;
use std::{process, thread, time};

fn go() -> Result<(), loragw::Error> {
    let concentrator = loragw::Gateway::open()?;

    let board_conf = loragw::BoardConf {
        lorawan_public: true,
        clksrc: 0,
    };
    concentrator.config_board(board_conf)?;

    concentrator.config_rx_rf(
        0,
        loragw::RxRFConf {
            enable: true,
            freq: 911_000_000,
            rssi_offset: -162.0,
            type_: loragw::RadioType::SX1257,
            tx_enable: false,
            tx_notch_freq: 0,
        },
    )?;

    concentrator.config_rx_rf(
        1,
        loragw::RxRFConf {
            enable: true,
            freq: 903_500_000,
            rssi_offset: -162.0,
            type_: loragw::RadioType::SX1257,
            tx_enable: false,
            tx_notch_freq: 0,
        },
    )?;

    // chan_multiSF_0
    concentrator.config_rx_if(
        0,
        loragw::RxIFConf {
            enable: true,
            chain: 0,
            freq: -400_000,
            bandwidth: loragw::Bandwidth::Undefined,
            spreading: loragw::Spreading::Undefined,
            sync_word_size: 0,
            sync_word: 0,
        },
    )?;

    // chan_multiSF_1
    concentrator.config_rx_if(
        1,
        loragw::RxIFConf {
            enable: true,
            chain: 0,
            freq: -200_000,
            bandwidth: loragw::Bandwidth::Undefined,
            spreading: loragw::Spreading::Undefined,
            sync_word_size: 0,
            sync_word: 0,
        },
    )?;

    // chan_multiSF_2
    concentrator.config_rx_if(
        2,
        loragw::RxIFConf {
            enable: true,
            chain: 0,
            freq: 0,
            bandwidth: loragw::Bandwidth::Undefined,
            spreading: loragw::Spreading::Undefined,
            sync_word_size: 0,
            sync_word: 0,
        },
    )?;

    // chan_multiSF_3
    concentrator.config_rx_if(
        3,
        loragw::RxIFConf {
            enable: true,
            chain: 0,
            freq: 200_000,
            bandwidth: loragw::Bandwidth::Undefined,
            spreading: loragw::Spreading::Undefined,
            sync_word_size: 0,
            sync_word: 0,
        },
    )?;

    // "chan_multiSF_4"
    concentrator.config_rx_if(
        4,
        loragw::RxIFConf {
            enable: true,
            chain: 1,
            freq: -400_000,
            bandwidth: loragw::Bandwidth::Undefined,
            spreading: loragw::Spreading::Undefined,
            sync_word_size: 0,
            sync_word: 0,
        },
    )?;

    // chan_multiSF_5
    concentrator.config_rx_if(
        5,
        loragw::RxIFConf {
            enable: true,
            chain: 1,
            freq: -200_000,
            bandwidth: loragw::Bandwidth::Undefined,
            spreading: loragw::Spreading::Undefined,
            sync_word_size: 0,
            sync_word: 0,
        },
    )?;

    // chan_multiSF_6
    concentrator.config_rx_if(
        6,
        loragw::RxIFConf {
            enable: true,
            chain: 1,
            freq: 0,
            bandwidth: loragw::Bandwidth::Undefined,
            spreading: loragw::Spreading::Undefined,
            sync_word_size: 0,
            sync_word: 0,
        },
    )?;

    // chan_multiSF_7
    concentrator.config_rx_if(
        7,
        loragw::RxIFConf {
            enable: true,
            chain: 1,
            freq: 200_000,
            bandwidth: loragw::Bandwidth::Undefined,
            spreading: loragw::Spreading::Undefined,
            sync_word_size: 0,
            sync_word: 0,
        },
    )?;

    // Lora STD
    concentrator.config_rx_if(
        8,
        loragw::RxIFConf {
            enable: true,
            chain: 0,
            freq: 300_000,
            bandwidth: loragw::Bandwidth::BW500kHz,
            spreading: loragw::Spreading::SF8,
            sync_word_size: 0,
            sync_word: 0,
        },
    )?;

    concentrator.start()?;

    loop {
        for pkt in concentrator.receive()? {
            println!("{:?}", pkt);
        }
        thread::sleep(time::Duration::from_millis(500));
    }
}

fn main() {
    env_logger::init();
    match go() {
        Ok(()) => process::exit(0),
        Err(e) => {
            eprintln!("ERROR: {}", e);
            process::exit(1);
        }
    }
}
