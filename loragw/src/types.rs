use super::error;
use std::convert::TryFrom;
use std::fmt;
use std::time;

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
    fn try_from(o: u32) -> Result<Self, error::Error> {
        Ok(match o {
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
    fn try_from(o: u32) -> Result<Self, error::Error> {
        Ok(match o {
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
    fn try_from(o: u32) -> Result<Self, error::Error> {
        Ok(match o {
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
    fn try_from(o: u32) -> Result<Self, error::Error> {
        match o {
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
    fn from(o: &BoardConf) -> Self {
        llg::lgw_conf_board_s {
            lorawan_public: o.lorawan_public,
            clksrc: o.clksrc as u8,
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
    fn from(o: &RxRFConf) -> Self {
        llg::lgw_conf_rxrf_s {
            enable: o.enable,
            freq_hz: o.freq,
            rssi_offset: o.rssi_offset,
            type_: o.type_ as u32,
            tx_enable: o.tx_enable,
            tx_notch_freq: o.tx_notch_freq,
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

impl From<&ChannelConf> for llg::lgw_conf_rxif_s {
    fn from(o: &ChannelConf) -> Self {
        match o {
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
    fn try_from(o: u32) -> Result<Self, Self::Error> {
        Ok(match o {
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
    fn try_from(o: &llg::lgw_pkt_rx_s) -> Result<Self, Self::Error> {
        const MOD_LORA: u8 = 0x10;
        const MOD_FSK: u8 = 0x20;
        Ok(match o.modulation {
            MOD_LORA => RxPacket::LoRa(RxPacketLoRa {
                freq: o.freq_hz,
                if_chain: o.if_chain,
                crc_check: CRCCheck::try_from(u32::from(o.status))?,
                timestamp: time::Duration::from_micros(u64::from(o.count_us)),
                radio: Radio::try_from(u32::from(o.rf_chain))?,
                bandwidth: Bandwidth::try_from(u32::from(o.bandwidth))?,
                spreading: Spreading::try_from(o.datarate)?,
                coderate: Coderate::try_from(u32::from(o.coderate))?,
                rssi: o.rssi,
                snr: o.snr,
                snr_min: o.snr_min,
                snr_max: o.snr_max,
                crc: o.crc,
                payload: o.payload[..o.size as usize].to_vec(),
            }),
            MOD_FSK => RxPacket::FSK(RxPacketFSK {
                freq: o.freq_hz,
                if_chain: o.if_chain,
                crc_check: CRCCheck::try_from(u32::from(o.status))?,
                timestamp: time::Duration::from_micros(u64::from(o.count_us)),
                radio: Radio::try_from(u32::from(o.rf_chain))?,
                datarate: o.datarate,
                rssi: o.rssi,
                crc: o.crc,
                payload: o.payload[..o.size as usize].to_vec(),
            }),
            _ => return Err(error::Error::Data),
        })
    }
}

/// A packet to transmit.
#[derive(Debug)]
pub struct TxPacket {
    /// Center frequency to transmit on.
    pub freq: u32,
    /// select on what event/time the TX is triggered.
    pub tx_mode: u8,
    /// Timestamp or delay (in uS) for when this packet should be
    /// transmitted.
    pub count_us: u32,
    /// RF chain to transmit on.
    pub rf_chain: u8,
    /// TX power (in dBm).
    pub rf_power: i8,
    /// Modulation type to use for this packet.
    pub modulation: u8,
    /// Modulation bandwidth (LoRa only).
    pub bandwidth: u8,
    /// Tx datarate (baudrate for FSK, SF for LoRa).
    pub datarate: u32,
    /// Error-correcting-code of the packet (LoRa only).
    pub coderate: u8,
    /// Invert signal polarity, for orthogonal downlinks (LoRa only).
    pub invert_pol: bool,
    /// Frequency deviation, in kHz (FSK only).
    pub f_dev: u8,
    /// Preamble length, 0 for default.
    pub preamble: u16,
    /// Do not send a CRC in the packet.
    pub no_crc: bool,
    /// Enable implicit header mode (LoRa), fixed length (FSK).
    pub no_header: bool,
    /// Arbitrary user-defined payload to transmit.
    pub payload: Vec<u8>,
}

impl TryFrom<TxPacket> for llg::lgw_pkt_tx_s {
    type Error = error::Error;

    fn try_from(o: TxPacket) -> Result<Self, error::Error> {
        if o.payload.len() > 256 {
            log::error!("attempt to send {} byte payload", o.payload.len());
            Err(error::Error::Size)
        } else {
            Ok(llg::lgw_pkt_tx_s {
                freq_hz: o.freq,
                tx_mode: o.tx_mode,
                count_us: o.count_us,
                rf_chain: o.rf_chain,
                rf_power: o.rf_power,
                modulation: o.modulation,
                bandwidth: o.bandwidth,
                datarate: o.datarate,
                coderate: o.coderate,
                invert_pol: o.invert_pol,
                f_dev: o.f_dev,
                preamble: o.preamble,
                no_crc: o.no_crc,
                no_header: o.no_header,
                size: o.payload.len() as u16,
                payload: {
                    let mut buf: [u8; 256] = [0u8; 256];
                    buf.copy_from_slice(o.payload.as_ref());
                    buf
                },
            })
        }
    }
}

/// Structure containing all gains of Tx chain.
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct TxGainLUT {
    /// Array of Tx gain struct.
    pub lut: [TxGain; 16],
    /// Number of LUT indexes.
    pub size: u8,
}
