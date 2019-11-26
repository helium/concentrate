use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, ffi::CString};
use toml;

#[cfg(feature = "sx1301")]
static DEFAULT_CFG_TOML: &str = include_str!("../default_config_sx1301.toml");

#[cfg(feature = "sx1302")]
static DEFAULT_CFG_TOML: &str = include_str!("../default_config_sx1302.toml");

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
            #[cfg(feature = "sx1302")]
            offset_i: 0,
            #[cfg(feature = "sx1302")]
            offset_q: 0,
            #[cfg(feature = "sx1302")]
            pwr_id: 0,
        }
    }
}
