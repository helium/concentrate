use super::error;
use std::convert::TryFrom;

/// Radio types that can be found on the LoRa Gateway
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum RadioType {
    None = 0,
    SX1255 = 1,
    SX1257 = 2,
    SX1272 = 3,
    SX1276 = 4,
}

/// Spreading factor
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum SpreadFactor {
    Undefined = 0x00,
    SF7 = 0x02,
    SF8 = 0x04,
    SF9 = 0x08,
    SF10 = 0x10,
    SF11 = 0x20,
    SF12 = 0x40,
    Multi = 0x7E,
}

/// Spreading factor
#[repr(u8)]
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

/// Configuration structure for board specificities
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BoardConf {
    /// Enable ONLY for *public* networks using the LoRa MAC protocol.
    pub lorawan_public: bool,
    /// Index of RF chain which provides clock to concentrator
    pub clksrc: u8,
}

/// Configuration structure for LBT channels
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LBTChanConf {
    pub freq_hz: u32,
    pub scan_time_us: u16,
}

/// Configuration structure for LBT specificities
#[repr(C)]
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
#[repr(C)]
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

/// Configuration structure for an If chain
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RxIFConf {
    /// enable or disable that If chain
    pub enable: bool,
    /// to which RF chain is that If chain associated
    pub chain: u8,
    /// center frequ of the If chain, relative to RF chain frequency
    pub freq: i32,
    /// Rx bandwidth, 0 for default
    pub bandwidth: Bandwidth,
    /// Rx spreading factor
    pub spread_factor: SpreadFactor,
    /// size of FSK sync word (number of bytes, 0 for default)
    pub sync_word_size: u8,
    /// FSK sync word (ALIGN RIGHT, eg. 0xC194C1)
    pub sync_word: u64,
}

/// Structure containing the metadata of a packet that was received and a pointer to the payload
#[derive(Debug)]
pub struct RxPacket {
    /// central frequency of the If chain
    pub freq_hz: u32,
    /// by which If chain was packet received
    pub if_chain: u8,
    /// status of the received packet
    pub status: u8,
    /// internal concentrator counter for timestamping, 1 microsecond resolution
    pub count_us: u32,
    /// through which RF chain the packet was received
    pub rf_chain: u8,
    /// modulation used by the packet
    pub modulation: u8,
    /// modulation bandwidth (LoRa only)
    pub bandwidth: u8,
    /// Rx datarate of the packet (SF for LoRa)
    pub datarate: u32,
    /// error-correcting code of the packet (LoRa only)
    pub coderate: u8,
    /// average packet RSSI in dB
    pub rssi: f32,
    /// average packet SNR, in dB (LoRa only)
    pub snr: f32,
    /// minimum packet SNR, in dB (LoRa only)
    pub snr_min: f32,
    /// maximum packet SNR, in dB (LoRa only)
    pub snr_max: f32,
    /// CRC that was received in the payload
    pub crc: u16,
    /// buffer containing the payload
    pub payload: Vec<u8>,
}

impl From<llg::lgw_pkt_rx_s> for RxPacket {
    fn from(o: llg::lgw_pkt_rx_s) -> Self {
        RxPacket {
            freq_hz: o.freq_hz,
            if_chain: o.if_chain,
            status: o.status,
            count_us: o.count_us,
            rf_chain: o.rf_chain,
            modulation: o.modulation,
            bandwidth: o.bandwidth,
            datarate: o.datarate,
            coderate: o.coderate,
            rssi: o.rssi,
            snr: o.snr,
            snr_min: o.snr_min,
            snr_max: o.snr_max,
            crc: o.crc,
            payload: o.payload[..o.size as usize].to_vec(),
        }
    }
}

/// Structure containing the configuration of a packet to send and a pointer to the payload
#[derive(Debug)]
pub struct TxPacket {
    /// center frequency of TX
    pub freq_hz: u32,
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
                freq_hz: o.freq_hz,
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
#[repr(C)]
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
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TxGainLUT {
    /// Array of Tx gain struct
    pub lut: [TxGain; 16],
    /// Number of LUT indexes
    pub size: u8,
}
