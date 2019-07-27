use crate::{cfg, error::AppResult, loragw};
use std::convert::{TryFrom, TryInto};

pub fn built_in_self_test() -> AppResult {
    let concentrator = loragw::Concentrator::open()?;
    let cfg = cfg::Config::from_str_or_default(None)?;

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

    concentrator.start()?;
    Ok(())
}
