use crate::error;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use toml;

static DEFAULT_CFG_TOML: &str = include_str!("../../default_config.toml");

/// Represents top-level configuration document.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub board: Board,
    pub radios: Option<Vec<Radio>>,
    pub multirate_channels: Option<Vec<MultirateLoraChannel>>,
}

impl Config {
    pub fn from_str_or_default(cfg: Option<&str>) -> error::Result<Self> {
        Self::from_str(cfg.unwrap_or(DEFAULT_CFG_TOML))
    }

    pub fn from_str(cfg: &str) -> error::Result<Self> {
        Ok(toml::from_str(cfg)?)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Board {
    pub lorawan_public: bool,
    pub clksrc: u32,
}

impl TryFrom<Board> for loragw::BoardConf {
    type Error = error::Error;
    fn try_from(other: Board) -> error::Result<loragw::BoardConf> {
        Ok(Self {
            lorawan_public: other.lorawan_public,
            clksrc: loragw::Radio::try_from(other.clksrc)?,
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
    type Error = error::Error;
    fn try_from(other: Radio) -> error::Result<Self> {
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
    type Error = error::Error;
    fn try_from(other: &MultirateLoraChannel) -> error::Result<loragw::ChannelConf> {
        Ok(loragw::ChannelConf::Multirate {
            radio: loragw::Radio::try_from(other.radio)?,
            freq: other.if_,
        })
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
                    freq: 911_500_000,
                    rssi_offset: -166.0,
                    type_: "SX1257".to_string(),
                    tx_enable: false,
                },
                Radio {
                    id: 1,
                    freq: 903_500_000,
                    rssi_offset: -166.0,
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
        };
        assert_eq!(reference_config, parsed_config);
    }
}
