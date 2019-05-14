use env_logger;
use loragw::{self, types};
use std::{process, thread, time};

fn go() -> Result<(), loragw::Error> {
    let concentrator = loragw::Gateway::open()?;

    let board_conf = types::BoardConf {
        lorawan_public: true,
        clksrc: 0,
    };
    concentrator.config_board(board_conf)?;

    let radio_0 = types::RxRFConf {
        enable: true,
        freq_hz: 902_700_000,
        rssi_offset: -162.0,
        type_: types::RadioType::SX1257,
        tx_enable: false,
        tx_notch_freq: 129_000,
    };
    concentrator.config_rx_rf(0, radio_0)?;

    let radio_1 = types::RxRFConf {
        enable: true,
        freq_hz: 903_500_000,
        rssi_offset: -162.0,
        type_: types::RadioType::SX1257,
        tx_enable: false,
        tx_notch_freq: 129_000,
    };
    concentrator.config_rx_rf(1, radio_1)?;

    concentrator.start()?;

    loop {
        for pkt in concentrator.receive()? {
            println!("{:?}", pkt);
        }
        thread::sleep(time::Duration::from_millis(500));
    }

    Ok(())
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
