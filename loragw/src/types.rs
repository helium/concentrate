use super::error;
use std::convert::TryFrom;
use std::fmt;
use std::time;

/// Radio types that can be found on the LoRa Gateway
#[derive(Debug, Clone, Copy)]
pub enum RadioType {
    None = 0,
    SX1255 = 1,
    SX1257 = 2,
    SX1272 = 3,
    SX1276 = 4,
}

/// Spreading factor
#[derive(Debug, Clone, Copy)]
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

/// Configured receive bandwidth
#[derive(Debug, Clone, Copy)]
pub enum Bandwidth {
    Undefined = 0,
    BW500kHz = 0x01,
    BW250kHz = 0x02,
    BW125kHz = 0x03,
    BW62_5kHz = 0x04,
    BW31_2kHz = 0x05,
    BW15_6Hz = 0x06,
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
            0x06 => Bandwidth::BW15_6Hz,
            0x07 => Bandwidth::BW7_8kHz,
            _ => return Err(error::Error::Data),
        })
    }
}

#[derive(Clone, Copy)]
pub enum Coderate {
    Undefined = 0,
    Cr4_5 = 0x01,
    Cr4_6 = 0x02,
    Cr4_7 = 0x03,
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
/// concentrator
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

/// Configuration structure for board specificities
#[derive(Debug, Clone)]
pub struct BoardConf {
    /// Enable ONLY for *public* networks using the LoRa MAC protocol.
    pub lorawan_public: bool,
    /// Index of RF chain which provides clock to concentrator
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

/// Configuration structure for LBT channels
#[derive(Debug, Clone)]
pub struct LBTChanConf {
    pub freq: u32,
    pub scan_time_us: u16,
}

/// Configuration structure for LBT specificities
#[derive(Debug, Clone)]
pub struct LBTConf {
    /// enable LBT
    pub enable: bool,
    /// RSSI threshold to detect if channel is busy or not (dBm)
    pub rssi_target: i8,
    /// number of LBT channels
    pub nb_channel: u8,
    pub channels: [LBTChanConf; 8],
    /// RSSI offset to be applied to SX127x RSSI values
    pub rssi_offset: i8,
}

/// Configuration structure for a RF chain
#[derive(Debug, Clone)]
pub struct RxRFConf {
    /// enable or disable that RF chain
    pub enable: bool,
    /// center frequency of the radio in Hz
    pub freq: u32,
    /// Board-specific RSSI correction factor
    pub rssi_offset: f32,
    /// Radio type for that RF chain (SX1255, SX1257...)
    pub type_: RadioType,
    /// enable or disable TX on that RF chain
    pub tx_enable: bool,
    /// TX notch filter frequency 126..250 Khz
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

/// Configuration structure for an If chain
#[derive(Debug, Clone)]
pub struct RxIFConf {
    /// enable or disable that If chain
    pub enable: bool,
    /// to which RF chain is that If chain associated
    pub radio: Radio,
    /// center frequ of the If chain, relative to RF chain frequency
    pub freq: i32,
    /// Rx bandwidth, 0 for default
    pub bandwidth: Bandwidth,
    /// Rx spreading factor
    pub spreading: Spreading,
    /// size of FSK sync word (number of bytes, 0 for default)
    pub sync_word_size: u8,
    /// FSK sync word (ALIGN RIGHT, eg. 0xC194C1)
    pub sync_word: u64,
}

impl From<&RxIFConf> for llg::lgw_conf_rxif_s {
    fn from(o: &RxIFConf) -> Self {
        llg::lgw_conf_rxif_s {
            enable: o.enable,
            rf_chain: o.radio as u8,
            freq_hz: o.freq,
            bandwidth: o.bandwidth as u8,
            datarate: o.spreading as u32,
            sync_word_size: o.sync_word_size,
            sync_word: o.sync_word,
            __bindgen_padding_0: Default::default(),
        }
    }
}

/// A _received_ Lora packet.
#[derive(Debug)]
pub struct LoraPkt {
    /// central frequency of the If chain
    pub freq: u32,
    /// by which If chain was packet received
    pub if_chain: u8,
    /// status of the received packet
    pub status: u8,
    /// internal concentrator counter for timestamping, 1 microsecond resolution
    pub timestamp: time::Duration,
    /// through which RF chain the packet was received
    pub radio: Radio,
    /// modulation bandwidth
    pub bandwidth: Bandwidth,
    /// Rx datarate of the packet
    pub spreading: Spreading,
    /// error-correcting code of the packet
    pub coderate: Coderate,
    /// average packet RSSI in dB
    pub rssi: f32,
    /// average packet SNR, in dB
    pub snr: f32,
    /// minimum packet SNR, in dB
    pub snr_min: f32,
    /// maximum packet SNR, in dB
    pub snr_max: f32,
    /// CRC that was received in the payload
    pub crc: u16,
    /// buffer containing the payload
    pub payload: Vec<u8>,
}

/// A _received_ FSK packet.
#[derive(Debug)]
pub struct FSKPkt {
    /// central frequency of the If chain
    pub freq: u32,
    /// by which If chain was packet received
    pub if_chain: u8,
    /// status of the received packet
    pub status: u8,
    /// internal concentrator counter for timestamping, 1 microsecond resolution
    pub timestamp: time::Duration,
    /// through which RF chain the packet was received
    pub radio: Radio,
    /// Rx datarate of the packet
    pub datarate: u32,
    /// average packet RSSI in dB
    pub rssi: f32,
    /// CRC that was received in the payload
    pub crc: u16,
    /// buffer containing the payload
    pub payload: Vec<u8>,
}

/// A _received_ packet.
#[derive(Debug)]
pub enum RxPkt {
    FSK(FSKPkt),
    Lora(LoraPkt),
}

impl TryFrom<&llg::lgw_pkt_rx_s> for RxPkt {
    type Error = error::Error;
    fn try_from(o: &llg::lgw_pkt_rx_s) -> Result<Self, Self::Error> {
        const MOD_LORA: u8 = 0x10;
        const MOD_FSK: u8 = 0x20;
        Ok(match o.modulation {
            MOD_LORA => RxPkt::Lora(LoraPkt {
                freq: o.freq_hz,
                if_chain: o.if_chain,
                status: o.status,
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
            MOD_FSK => RxPkt::FSK(FSKPkt {
                freq: o.freq_hz,
                if_chain: o.if_chain,
                status: o.status,
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

/// Structure containing the configuration of a packet to send and a pointer to the payload
#[derive(Debug)]
pub struct TxPacket {
    /// center frequency of TX
    pub freq: u32,
    /// select on what event/time the TX is triggered
    pub tx_mode: u8,
    /// timestamp or delay in microseconds for TX trigger
    pub count_us: u32,
    /// through which RF chain will the packet be sent
    pub rf_chain: u8,
    /// TX power, in dBm
    pub rf_power: i8,
    /// modulation to use for the packet
    pub modulation: u8,
    /// modulation bandwidth (LoRa only)
    pub bandwidth: u8,
    /// TX datarate (baudrate for FSK, SF for LoRa)
    pub datarate: u32,
    /// error-correcting code of the packet (LoRa only)
    pub coderate: u8,
    /// invert signal polarity, for orthogonal downlinks (LoRa only)
    pub invert_pol: bool,
    /// frequency deviation, in kHz (FSK only)
    pub f_dev: u8,
    /// set the preamble length, 0 for default
    pub preamble: u16,
    /// if true, do not send a CRC in the packet
    pub no_crc: bool,
    /// if true, enable implicit header mode (LoRa), fixed length (FSK)
    pub no_header: bool,
    /// buffer containing the payload
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

/// Structure containing all gains of Tx chain
#[derive(Debug, Clone)]
pub struct TxGain {
    /// 2 bits, control of the digital gain of SX1301
    pub dig_gain: u8,
    /// 2 bits, control of the external PA (SX1301 I/O)
    pub pa_gain: u8,
    /// 2 bits, control of the radio DAC
    pub dac_gain: u8,
    /// 4 bits, control of the radio mixer
    pub mix_gain: u8,
    /// measured TX power at the board connector, in dBm
    pub rf_power: i8,
}

/// Structure defining the Tx gain LUT
#[derive(Debug, Clone)]
pub struct TxGainLUT {
    /// Array of Tx gain struct
    pub lut: [TxGain; 16],
    /// Number of LUT indexes
    pub size: u8,
}
