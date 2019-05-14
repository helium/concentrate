use env_logger;
use loragw::{self, types::*};
use std::{process, thread, time};

fn go() -> Result<(), loragw::Error> {
    let concentrator = loragw::Gateway::open()?;

    let board_conf = BoardConf {
        lorawan_public: true,
        clksrc: 0,
    };
    concentrator.config_board(board_conf)?;

    let radio_0 = RxRFConf {
        enable: true,
        freq: 902_700_000,
        rssi_offset: -162.0,
        type_: RadioType::SX1257,
        tx_enable: false,
        tx_notch_freq: 0,
    };
    concentrator.config_rx_rf(0, radio_0)?;

    let radio_1 = RxRFConf {
        enable: true,
        freq: 903_500_000,
        rssi_offset: -162.0,
        type_: RadioType::SX1257,
        tx_enable: false,
        tx_notch_freq: 0,
    };
    concentrator.config_rx_rf(1, radio_1)?;

    // Lora STD
    concentrator.config_rx_if(
        8,
        RxIFConf {
            enable: true,
            chain: 0,
            freq: 300_000,
            bandwidth: Bandwidth::BW500kHz,
            spread_factor: SpreadFactor::SF8,
            sync_word_size: 0,
            sync_word: 0,
        },
    )?;

    // FSK
    concentrator.config_rx_if(
        9,
        RxIFConf {
            enable: false,
            chain: 0,
            freq: 0,
            bandwidth: Bandwidth::Undefined,
            spread_factor: SpreadFactor::Undefined,
            sync_word_size: 0,
            sync_word: 0,
        },
    )?;

    // "chan_multiSF_0": {
    //     "enable": true,
    //     "if": -400000,
    //     "radio": 0
    // },
    concentrator.config_rx_if(
        0,
        RxIFConf {
            enable: true,
            chain: 0,
            freq: -400_000,
            bandwidth: Bandwidth::Undefined,
            spread_factor: SpreadFactor::Undefined,
            sync_word_size: 0,
            sync_word: 0,
        },
    )?;

    // "chan_multiSF_1": {
    //     "enable": true,
    //     "if": -200000,
    //     "radio": 0
    // },
    concentrator.config_rx_if(
        1,
        RxIFConf {
            enable: true,
            chain: 0,
            freq: -200_000,
            bandwidth: Bandwidth::Undefined,
            spread_factor: SpreadFactor::Undefined,
            sync_word_size: 0,
            sync_word: 0,
        },
    )?;

    // "chan_multiSF_2": {
    //     "enable": true,
    //     "if": 0,
    //     "radio": 0
    // },
    concentrator.config_rx_if(
        2,
        RxIFConf {
            enable: true,
            chain: 0,
            freq: 0,
            bandwidth: Bandwidth::Undefined,
            spread_factor: SpreadFactor::Undefined,
            sync_word_size: 0,
            sync_word: 0,
        },
    )?;

    // "chan_multiSF_3": {
    //     "enable": true,
    //     "if": 200000,
    //     "radio": 0
    // },
    concentrator.config_rx_if(
        3,
        RxIFConf {
            enable: true,
            chain: 0,
            freq: 200_000,
            bandwidth: Bandwidth::Undefined,
            spread_factor: SpreadFactor::Undefined,
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
