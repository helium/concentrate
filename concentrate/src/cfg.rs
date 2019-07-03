use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::ffi::CString;
use toml;

static DEFAULT_CFG_TOML: &str = include_str!("../../default_config.toml");

/// Represents top-level configuration document.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub board: Board,
    pub radios: Option<Vec<Radio>>,
    pub multirate_channels: Option<Vec<MultirateLoraChannel>>,
    pub tx_gains: Option<Vec<TxGain>>,
}

impl Config {
    pub fn from_str_or_default(cfg: Option<&str>) -> AppResult<Self> {
        Self::from_str(cfg.unwrap_or(DEFAULT_CFG_TOML))
    }

    pub fn from_str(cfg: &str) -> AppResult<Self> {
        Ok(toml::from_str(cfg)?)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Board {
    pub lorawan_public: bool,
    pub clksrc: u32,
    pub spidev_path: CString,
}

impl TryFrom<Board> for loragw::BoardConf {
    type Error = AppError;
    fn try_from(other: Board) -> AppResult<loragw::BoardConf> {
        Ok(Self {
            lorawan_public: other.lorawan_public,
            clksrc: loragw::Radio::try_from(other.clksrc)?,
            spidev_path: other.spidev_path,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Radio {
    pub id: u32,
    pub freq: u32,
    pub rssi_offset: f32,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub type_: String,
    pub tx_enable: bool,
}

impl TryFrom<Radio> for loragw::RxRFConf {
    type Error = AppError;
    fn try_from(other: Radio) -> AppResult<Self> {
        Ok(loragw::RxRFConf {
            radio: loragw::Radio::try_from(other.id)?,
            enable: true,
            freq: other.freq,
            rssi_offset: other.rssi_offset,
            type_: loragw::RadioType::try_from(other.type_.as_ref())?,
            tx_enable: other.tx_enable,
            tx_notch_freq: 0,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MultirateLoraChannel {
    radio: u32,
    #[serde(rename(serialize = "if", deserialize = "if"))]
    if_: i32,
}

impl TryFrom<&MultirateLoraChannel> for loragw::ChannelConf {
    type Error = AppError;
    fn try_from(other: &MultirateLoraChannel) -> AppResult<loragw::ChannelConf> {
        Ok(loragw::ChannelConf::Multirate {
            radio: loragw::Radio::try_from(other.radio)?,
            freq: other.if_,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TxGain {
    #[serde(rename(serialize = "dbm", deserialize = "dbm"))]
    pub rf_power: i8,
    #[serde(rename(serialize = "dig", deserialize = "dig"))]
    pub dig_gain: u8,
    #[serde(rename(serialize = "pa", deserialize = "pa"))]
    pub pa_gain: u8,
    #[serde(rename(serialize = "mix", deserialize = "mix"))]
    pub mix_gain: u8,
}

impl From<TxGain> for loragw::TxGain {
    fn from(other: TxGain) -> loragw::TxGain {
        loragw::TxGain {
            dig_gain: other.dig_gain,
            pa_gain: other.pa_gain,
            dac_gain: 3,
            mix_gain: other.mix_gain,
            rf_power: other.rf_power,
            offset_i: 0,
            offset_q: 0,
            pwr_id: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_radioconfig() {
        let parsed_config = toml::from_str::<Config>(DEFAULT_CFG_TOML).unwrap();
        let reference_config = Config {
            board: Board {
                lorawan_public: false,
                clksrc: 1,
            },
            radios: Some(vec![
                Radio {
                    id: 0,
                    freq: 916_600_000,
                    rssi_offset: -169.0,
                    type_: "SX1257".to_string(),
                    tx_enable: true,
                },
                Radio {
                    id: 1,
                    freq: 920_600_000,
                    rssi_offset: -169.0,
                    type_: "SX1257".to_string(),
                    tx_enable: false,
                },
            ]),
            multirate_channels: Some(vec![
                MultirateLoraChannel {
                    radio: 1,
                    if_: -400_000,
                },
                MultirateLoraChannel {
                    radio: 1,
                    if_: -200_000,
                },
                MultirateLoraChannel { radio: 1, if_: 0 },
                MultirateLoraChannel {
                    radio: 0,
                    if_: -400_000,
                },
                MultirateLoraChannel {
                    radio: 0,
                    if_: -200_000,
                },
                MultirateLoraChannel { radio: 0, if_: 0 },
                MultirateLoraChannel {
                    radio: 0,
                    if_: 200_000,
                },
                MultirateLoraChannel {
                    radio: 0,
                    if_: 400_000,
                },
            ]),
            tx_gains: Some(vec![
                TxGain {
                    rf_power: -11,
                    dig_gain: 3,
                    pa_gain: 0,
                    mix_gain: 8,
                },
                TxGain {
                    rf_power: -7,
                    dig_gain: 3,
                    pa_gain: 0,
                    mix_gain: 10,
                },
                TxGain {
                    rf_power: -4,
                    dig_gain: 1,
                    pa_gain: 0,
                    mix_gain: 10,
                },
                TxGain {
                    rf_power: -1,
                    dig_gain: 2,
                    pa_gain: 0,
                    mix_gain: 14,
                },
                TxGain {
                    rf_power: 3,
                    dig_gain: 3,
                    pa_gain: 1,
                    mix_gain: 10,
                },
                TxGain {
                    rf_power: 9,
                    dig_gain: 2,
                    pa_gain: 1,
                    mix_gain: 12,
                },
                TxGain {
                    rf_power: 10,
                    dig_gain: 1,
                    pa_gain: 1,
                    mix_gain: 12,
                },
                TxGain {
                    rf_power: 11,
                    dig_gain: 0,
                    pa_gain: 1,
                    mix_gain: 12,
                },
                TxGain {
                    rf_power: 12,
                    dig_gain: 2,
                    pa_gain: 1,
                    mix_gain: 14,
                },
                TxGain {
                    rf_power: 15,
                    dig_gain: 1,
                    pa_gain: 2,
                    mix_gain: 11,
                },
                TxGain {
                    rf_power: 18,
                    dig_gain: 1,
                    pa_gain: 2,
                    mix_gain: 13,
                },
                TxGain {
                    rf_power: 19,
                    dig_gain: 2,
                    pa_gain: 2,
                    mix_gain: 15,
                },
                TxGain {
                    rf_power: 22,
                    dig_gain: 2,
                    pa_gain: 3,
                    mix_gain: 10,
                },
                TxGain {
                    rf_power: 23,
                    dig_gain: 1,
                    pa_gain: 3,
                    mix_gain: 10,
                },
                TxGain {
                    rf_power: 28,
                    dig_gain: 1,
                    pa_gain: 3,
                    mix_gain: 14,
                },
            ]),
        };
        assert_eq!(reference_config, parsed_config);
    }
}
