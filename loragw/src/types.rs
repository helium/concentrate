use crate::error;
use llg;
use std::{convert::TryFrom, fmt, time};

const MOD_LORA: u8 = 0x10;
const MOD_FSK: u8 = 0x20;

/// Radio types that can be found on the LoRa concentrator.
#[derive(Debug, Clone, Copy)]
#[allow(missing_docs)]
pub enum RadioType {
    None = 0,
    SX1255 = 1,
    SX1257 = 2,
    SX1272 = 3,
    SX1276 = 4,
}

impl TryFrom<&str> for RadioType {
    type Error = error::Error;
    fn try_from(s: &str) -> Result<Self, error::Error> {
        Ok(match s {
            "None" => RadioType::None,
            "SX1255" => RadioType::SX1255,
            "SX1257" => RadioType::SX1257,
            "SX1272" => RadioType::SX1272,
            "SX1276" => RadioType::SX1276,
            _ => return Err(error::Error::Data),
        })
    }
}

/// Spreading factor.
#[derive(Debug, Clone, Copy)]
#[allow(missing_docs)]
pub enum Spreading {
    Undefined = 0x00,
    SF7 = 0x02,
    SF8 = 0x04,
    SF9 = 0x08,
    SF10 = 0x10,
    SF11 = 0x20,
    SF12 = 0x40,
    Multi = 0x7E,
}

impl TryFrom<u32> for Spreading {
    type Error = error::Error;
    fn try_from(other: u32) -> Result<Self, error::Error> {
        Ok(match other {
            0x00 => Spreading::Undefined,
            0x02 => Spreading::SF7,
            0x04 => Spreading::SF8,
            0x08 => Spreading::SF9,
            0x10 => Spreading::SF10,
            0x20 => Spreading::SF11,
            0x40 => Spreading::SF12,
            0x7E => Spreading::Multi,
            _ => return Err(error::Error::Data),
        })
    }
}

/// Configured receive bandwidth.
#[derive(Debug, Clone, Copy)]
pub enum Bandwidth {
    /// Auto bandwidth.
    Undefined = 0,
    /// 500 kHz.
    BW500kHz = 0x01,
    /// 250 kHz.
    BW250kHz = 0x02,
    /// 125 kHz.
    BW125kHz = 0x03,
    /// 62.5 kHz
    BW62_5kHz = 0x04,
    /// 31.2 kHz.
    BW31_2kHz = 0x05,
    /// 15.6 kHz.
    BW15_6kHz = 0x06,
    /// 7.8 kHz.
    BW7_8kHz = 0x07,
}

impl TryFrom<u32> for Bandwidth {
    type Error = error::Error;
    fn try_from(other: u32) -> Result<Self, error::Error> {
        Ok(match other {
            0 => Bandwidth::Undefined,
            0x01 => Bandwidth::BW500kHz,
            0x02 => Bandwidth::BW250kHz,
            0x03 => Bandwidth::BW125kHz,
            0x04 => Bandwidth::BW62_5kHz,
            0x05 => Bandwidth::BW31_2kHz,
            0x06 => Bandwidth::BW15_6kHz,
            0x07 => Bandwidth::BW7_8kHz,
            _ => return Err(error::Error::Data),
        })
    }
}

/// Configured error correction code rate.
#[derive(Clone, Copy)]
pub enum Coderate {
    /// Auto code rate.
    Undefined = 0,
    /// 4/5.
    Cr4_5 = 0x01,
    /// 4/6.
    Cr4_6 = 0x02,
    /// 4/7.
    Cr4_7 = 0x03,
    /// 4/8.
    Cr4_8 = 0x04,
}

impl TryFrom<u32> for Coderate {
    type Error = error::Error;
    fn try_from(other: u32) -> Result<Self, error::Error> {
        Ok(match other {
            0 => Coderate::Undefined,
            0x01 => Coderate::Cr4_5,
            0x02 => Coderate::Cr4_6,
            0x03 => Coderate::Cr4_7,
            0x04 => Coderate::Cr4_8,
            _ => return Err(error::Error::Data),
        })
    }
}

impl fmt::Debug for Coderate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Coderate::Undefined => "Undefined",
                Coderate::Cr4_5 => "4/5",
                Coderate::Cr4_6 => "4/6",
                Coderate::Cr4_7 => "4/7",
                Coderate::Cr4_8 => "4/8",
            }
        )
    }
}

/// Represents one of two possible front-end radios connected to the
/// concentrator.
#[derive(Debug, Clone, Copy)]
pub enum Radio {
    /// Radio 0
    R0 = 0,
    /// Radio 1
    R1 = 1,
}

impl TryFrom<u32> for Radio {
    type Error = error::Error;
    fn try_from(other: u32) -> Result<Self, error::Error> {
        match other {
            0 => Ok(Radio::R0),
            1 => Ok(Radio::R1),
            _ => Err(error::Error::Data),
        }
    }
}

/// Board-specific configuration.
#[derive(Debug, Clone)]
pub struct BoardConf {
    /// Enable ONLY for *public* networks using the LoRa MAC protocol.
    pub lorawan_public: bool,
    /// Index of RF chain which provides clock to concentrator.
    pub clksrc: Radio,
}

impl From<&BoardConf> for llg::lgw_conf_board_s {
    fn from(other: &BoardConf) -> Self {
        llg::lgw_conf_board_s {
            lorawan_public: other.lorawan_public,
            clksrc: other.clksrc as u8,
        }
    }
}

/// Configuration structure for LBT channels.
#[derive(Debug, Clone)]
pub struct LBTChanConf {
    /// Center frequency of this channel.
    pub freq: u32,
    /// Time (in uS) to listen for.
    pub scan_time_us: u16,
}

/// Listen-before-talk configuration.
#[derive(Debug, Clone)]
pub struct LBTConf {
    /// Enable LBT.
    pub enable: bool,
    /// RSSI threshold for deciding if channel is currently busy.
    pub rssi_target: i8,
    /// Number of LBT chanels.
    pub nb_channel: u8,
    /// Channel map.
    pub channels: [LBTChanConf; 8],
    /// RSSI offset to be applied to SX127x RSSI values.
    pub rssi_offset: i8,
}

/// RF chain configuration.
#[derive(Debug, Clone)]
pub struct RxRFConf {
    /// The radio we are configuring.
    pub radio: Radio,
    /// Enable this RF chain.
    pub enable: bool,
    /// Tune this chain to this frequency.
    pub freq: u32,
    /// Board-specific RSSI correction factor.
    pub rssi_offset: f32,
    /// Radio model of this chain.
    pub type_: RadioType,
    /// Enable transmission on this chain.
    pub tx_enable: bool,
    /// TX notch filter center frequency 126..250 Khz.
    pub tx_notch_freq: u32,
}

impl From<&RxRFConf> for llg::lgw_conf_rxrf_s {
    fn from(other: &RxRFConf) -> Self {
        llg::lgw_conf_rxrf_s {
            enable: other.enable,
            freq_hz: other.freq,
            rssi_offset: other.rssi_offset,
            type_: other.type_ as u32,
            tx_enable: other.tx_enable,
            tx_notch_freq: other.tx_notch_freq,
        }
    }
}

/// Modem and IF configuration.
#[derive(Debug, Clone)]
pub enum ChannelConf {
    /// Disable this channel.
    Disable,
    /// Standard (fixed bandwidth and spreading) LoRa channel.
    Fixed {
        /// IQ sample source for this channel.
        radio: Radio,
        /// Center frequency for this channel (relative to `radio`'s center frequency).
        freq: i32,
        /// Receive bandwidth.
        bandwidth: Bandwidth,
        /// Receive spreading factor.
        spreading: Spreading,
    },
    /// Multirate LoRa channel.
    ///
    /// A multirate channel can automatically detect, demodulate, and
    /// decode packets transmitted with varying valid LoRa schemes.
    Multirate {
        /// IQ sample source for this channel.
        radio: Radio,
        /// Center frequency for this channel (relative to `radio`'s center frequency).
        freq: i32,
    },
    /// Frequency-shift-keying channel.
    FSK {
        /// IQ sample source for this channel.
        radio: Radio,
        /// Center frequency for this channel (relative to `radio`'s center frequency).
        freq: i32,
        /// Receive bandwidth.
        bandwidth: Bandwidth,
        /// Receive data rate.
        datarate: u32,
        /// Size of FSK synchronization word (number of bytes, 0 for default).
        sync_word_size: u8,
        /// Synchronization word (right-aligned, eg. 0xC194C1).
        sync_word: u64,
    },
}

#[allow(clippy::needless_update)]
impl From<&ChannelConf> for llg::lgw_conf_rxif_s {
    fn from(other: &ChannelConf) -> Self {
        match other {
            ChannelConf::Disable => llg::lgw_conf_rxif_s {
                enable: false,
                rf_chain: 0,
                freq_hz: 0,
                bandwidth: 0,
                datarate: 0,
                sync_word_size: 0,
                sync_word: 0,
                ..Default::default()
            },
            &ChannelConf::Fixed {
                radio,
                freq,
                bandwidth,
                spreading,
            } => llg::lgw_conf_rxif_s {
                enable: true,
                rf_chain: radio as u8,
                freq_hz: freq,
                bandwidth: bandwidth as u8,
                datarate: spreading as u32,
                sync_word_size: 0,
                sync_word: 0,
                ..Default::default()
            },
            &ChannelConf::Multirate { radio, freq } => llg::lgw_conf_rxif_s {
                enable: true,
                rf_chain: radio as u8,
                freq_hz: freq,
                bandwidth: Bandwidth::Undefined as u8,
                datarate: Spreading::Undefined as u32,
                sync_word_size: 0,
                sync_word: 0,
                ..Default::default()
            },
            &ChannelConf::FSK {
                radio,
                freq,
                bandwidth,
                datarate,
                sync_word_size,
                sync_word,
            } => llg::lgw_conf_rxif_s {
                enable: true,
                rf_chain: radio as u8,
                freq_hz: freq,
                bandwidth: bandwidth as u8,
                datarate,
                sync_word_size,
                sync_word,
                ..Default::default()
            },
        }
    }
}

/// Status of CRC check returned with received packets.
#[derive(Debug, Clone, Copy)]
pub enum CRCCheck {
    /// The received packet was transmitted without a CRC.
    NoCRC,
    /// The received CRC and calculated CRC differ.
    Fail,
    /// The received CRC and calculated CRC are equal.
    Pass,
}

impl TryFrom<u32> for CRCCheck {
    type Error = error::Error;
    fn try_from(other: u32) -> Result<Self, Self::Error> {
        Ok(match other {
            0x01 => CRCCheck::NoCRC,
            0x11 => CRCCheck::Fail,
            0x10 => CRCCheck::Pass,
            _ => return Err(error::Error::Data),
        })
    }
}

/// A received LoRa-modulated packet.
#[derive(Debug)]
pub struct RxPacketLoRa {
    /// Center frequency of the channel this packet was received on.
    pub freq: u32,
    /// Channel this packet packet was received on.
    pub if_chain: u8,
    /// Status of CRC check.
    pub crc_check: CRCCheck,
    /// 1uS-resolution timestamp derived from concentrator's internal counter.
    pub timestamp: time::Duration,
    /// RF chain this packet was received on.
    pub radio: Radio,
    /// Modulation bandwidth.
    pub bandwidth: Bandwidth,
    /// Spreading factor of this packet.
    pub spreading: Spreading,
    /// Error Correcting Code rate of this packet.
    pub coderate: Coderate,
    /// Average packet RSSI in dB.
    pub rssi: f32,
    /// Average packet SNR, in dB.
    pub snr: f32,
    /// Minimum packet SNR, in dB.
    pub snr_min: f32,
    /// Maximum packet SNR, in dB.
    pub snr_max: f32,
    /// CRC that was declared in this packet's payload.
    ///
    /// A CRC-check mismatch indicates that this packet was corrupted in flight.
    pub crc: u16,
    /// This packet's payload.
    pub payload: Vec<u8>,
}

/// A received FSK-modulated packet.
#[derive(Debug)]
pub struct RxPacketFSK {
    /// Center frequency of the channel this packet was received on.
    pub freq: u32,
    /// Channel this packet packet was received on.
    pub if_chain: u8,
    /// Status of CRC check.
    pub crc_check: CRCCheck,
    /// 1uS-resolution timestamp derived from concentrator's internal counter.
    pub timestamp: time::Duration,
    /// RF chain this packet was received on.
    pub radio: Radio,
    /// Datarate of this packet.
    pub datarate: u32,
    /// Average packet RSSI, in dB.
    pub rssi: f32,
    /// CRC that was declared in this packet's payload.
    ///
    /// A CRC-check mismatch indicates that this packet was corrupted in flight.
    pub crc: u16,
    /// This packet's payload.
    pub payload: Vec<u8>,
}

/// A received packet.
#[derive(Debug)]
pub enum RxPacket {
    /// This packet was transmitted using FSK modulation.
    FSK(RxPacketFSK),
    /// This packet was transmitted using LoRa modulation.
    LoRa(RxPacketLoRa),
}

impl TryFrom<&llg::lgw_pkt_rx_s> for RxPacket {
    type Error = error::Error;
    fn try_from(other: &llg::lgw_pkt_rx_s) -> Result<Self, Self::Error> {
        Ok(match other.modulation {
            MOD_LORA => RxPacket::LoRa(RxPacketLoRa {
                freq: other.freq_hz,
                if_chain: other.if_chain,
                crc_check: CRCCheck::try_from(u32::from(other.status))?,
                timestamp: time::Duration::from_micros(u64::from(other.count_us)),
                radio: Radio::try_from(u32::from(other.rf_chain))?,
                bandwidth: Bandwidth::try_from(u32::from(other.bandwidth))?,
                spreading: Spreading::try_from(other.datarate)?,
                coderate: Coderate::try_from(u32::from(other.coderate))?,
                rssi: other.rssi,
                snr: other.snr,
                snr_min: other.snr_min,
                snr_max: other.snr_max,
                crc: other.crc,
                payload: other.payload[..other.size as usize].to_vec(),
            }),
            MOD_FSK => RxPacket::FSK(RxPacketFSK {
                freq: other.freq_hz,
                if_chain: other.if_chain,
                crc_check: CRCCheck::try_from(u32::from(other.status))?,
                timestamp: time::Duration::from_micros(u64::from(other.count_us)),
                radio: Radio::try_from(u32::from(other.rf_chain))?,
                datarate: other.datarate,
                rssi: other.rssi,
                crc: other.crc,
                payload: other.payload[..other.size as usize].to_vec(),
            }),
            _ => return Err(error::Error::Data),
        })
    }
}

/// Specifies when to send a `TxPacket`
#[derive(Debug, Clone, Copy)]
pub enum TxMode {
    /// Do not delay.
    ///
    /// There are still delays incurred from SPI communication and
    /// analog circuitry settling.
    Immediate,
    /// Send when concentrator's internal counter equals the time specified.
    Timestamp(time::Duration),
    /// Send at specified duration after the next GPS pulse-per-second transition.
    PPS(time::Duration),
}

impl From<TxMode> for (u8, u32) {
    fn from(other: TxMode) -> (u8, u32) {
        use crate::TxMode::*;
        match other {
            Immediate => (0, 0),
            Timestamp(delay) => (1, delay.as_micros() as u32),
            PPS(delay) => (2, delay.as_micros() as u32),
        }
    }
}

/// Holds either a LoRa or FSK packet.
#[derive(Debug, Clone)]
pub enum TxPacket {
    /// A transmittable LoRa packet.
    LoRa(TxPacketLoRa),
    /// A transmittable FSK packet.
    FSK(TxPacketFSK),
}

/// A transmittable LoRa packet.
#[derive(Debug, Clone)]
pub struct TxPacketLoRa {
    /// Center frequency to transmit on.
    pub freq: u32,
    /// When to send this packet.
    pub mode: TxMode,
    /// Which radio to transmit on.
    pub radio: Radio,
    /// TX power (in dBm).
    pub power: i8,
    /// Modulation bandwidth.
    pub bandwidth: Bandwidth,
    /// Spreading factor to use with this packet.
    pub spreading: Spreading,
    /// Error-correcting-code of the packet.
    pub coderate: Coderate,
    /// Invert signal polarity for orthogonal downlinks.
    pub invert_polarity: bool,
    /// Preamble length.
    ///
    /// Use `None` for default.
    pub preamble: Option<u16>,
    /// Do not send a CRC in the packet.
    pub omit_crc: bool,
    /// Enable implicit header mode.
    pub implicit_header: bool,
    /// Arbitrary user-defined payload to transmit.
    pub payload: Vec<u8>,
}

/// A transmittable LoRa packet.
#[derive(Debug, Clone)]
pub struct TxPacketFSK {
    /// Center frequency to transmit on.
    pub freq: u32,
    /// When to send this packet.
    pub mode: TxMode,
    /// Which radio to transmit on.
    pub radio: Radio,
    /// TX power (in dBm).
    pub power: i8,
    /// Datarate in bits/second.
    pub datarate: u32,
    /// Frequency deviation, in kHz.
    pub deviation: u8,
    /// Preamble length.
    ///
    /// Use `None` for default.
    pub preamble: Option<u16>,
    /// Do not send a CRC in the packet.
    pub omit_crc: bool,
    /// Enable fixed length packet.
    pub fixed_len: bool,
    /// Arbitrary user-defined payload to transmit.
    pub payload: Vec<u8>,
}

impl TryFrom<TxPacket> for llg::lgw_pkt_tx_s {
    type Error = error::Error;

    fn try_from(other: TxPacket) -> Result<Self, error::Error> {
        match other {
            TxPacket::LoRa(other) => {
                if other.payload.len() > 256 {
                    log::error!("attempt to send {} byte payload", other.payload.len());
                    Err(error::Error::Size)
                } else {
                    let (mode, delay) = other.mode.into();
                    Ok(llg::lgw_pkt_tx_s {
                        freq_hz: other.freq,
                        tx_mode: mode,
                        count_us: delay,
                        rf_chain: other.radio as u8,
                        rf_power: other.power,
                        modulation: MOD_LORA,
                        bandwidth: other.bandwidth as u8,
                        datarate: other.spreading as u32,
                        coderate: other.coderate as u8,
                        invert_pol: other.invert_polarity,
                        f_dev: 0,
                        preamble: other.preamble.unwrap_or(0),
                        no_crc: other.omit_crc,
                        no_header: other.implicit_header,
                        size: other.payload.len() as u16,
                        payload: {
                            let mut buf: [u8; 256] = [0u8; 256];
                            buf[..other.payload.len()].copy_from_slice(other.payload.as_ref());
                            buf
                        },
                    })
                }
            }
            TxPacket::FSK(other) => {
                if other.payload.len() > 256 {
                    log::error!("attempt to send {} byte payload", other.payload.len());
                    Err(error::Error::Size)
                } else {
                    let (mode, delay) = other.mode.into();
                    Ok(llg::lgw_pkt_tx_s {
                        freq_hz: other.freq,
                        tx_mode: mode,
                        count_us: delay,
                        rf_chain: other.radio as u8,
                        rf_power: other.power,
                        modulation: MOD_FSK,
                        bandwidth: 0,
                        datarate: other.datarate,
                        coderate: Coderate::Undefined as u8,
                        invert_pol: false,
                        f_dev: other.deviation,
                        preamble: other.preamble.unwrap_or(0),
                        no_crc: other.omit_crc,
                        no_header: other.fixed_len,
                        size: other.payload.len() as u16,
                        payload: {
                            let mut buf: [u8; 256] = [0u8; 256];
                            buf[..other.payload.len()].copy_from_slice(other.payload.as_ref());
                            buf
                        },
                    })
                }
            }
        }
    }
}

/// Structure containing all gains of Tx chain.
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TxGain {
    /// Control of the digital gain of SX1301 (2 bits).
    pub dig_gain: u8,
    /// Control of the external PA (SX1301 I/O) (2 bits).
    pub pa_gain: u8,
    /// Control of the radio DAC (2 bits).
    pub dac_gain: u8,
    /// control of the radio mixer (4 bits).
    pub mix_gain: u8,
    /// Measured TX power at the board connector (in dBm).
    pub rf_power: i8,
}

/// Tx gain look-up-table.
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TxGainLUT {
    /// Array of Tx gain struct.
    pub lut: [TxGain; 16],
    /// Number of LUT indexes.
    pub size: u8,
}

/// Concentrator's current TX availability.
#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TxStatus {
    /// TX modem disabled, it will ignore commands.
    Off = 1,
    /// TX modem is free, ready to receive a command.
    Free = 2,
    /// TX modem is loaded, ready to send the packet after an event and/or delay.
    Scheduled = 3,
    /// TX modem is emitting.
    Transmitting = 4,
}

impl TryFrom<u8> for TxStatus {
    type Error = error::Error;
    fn try_from(other: u8) -> Result<Self, error::Error> {
        Ok(match other {
            1 => TxStatus::Off,
            2 => TxStatus::Free,
            3 => TxStatus::Scheduled,
            4 => TxStatus::Transmitting,
            _ => return Err(error::Error::Data),
        })
    }
}

/// Concentrator's current RX availability.
#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RxStatus {
    /// RX modem is disabled, it will ignore commands.
    Off = 1,
    /// RX modem is receiving.
    On = 2,
    /// RX is suspended while a TX is ongoing.
    Suspended = 3,
}

impl TryFrom<u8> for RxStatus {
    type Error = error::Error;
    fn try_from(other: u8) -> Result<Self, error::Error> {
        Ok(match other {
            1 => RxStatus::Off,
            2 => RxStatus::On,
            3 => RxStatus::Suspended,
            _ => return Err(error::Error::Data),
        })
    }
}
