use llg::*;

/// Radio types that can be found on the LoRa Gateway
#[derive(Debug, Clone, Copy)]
pub enum RadioType {
    None,
    SX1255,
    SX1257,
    SX1272,
    SX1276,
}

/// Configuration structure for board specificities
#[derive(Debug)]
pub struct BoardConf {
    /// Enable ONLY for *public* networks using the LoRa MAC protocol.
    pub lorawan_public: bool,
    /// Index of RF chain which provides clock to concentrator
    pub clksrc: usize,
}

/// Configuration structure for LBT channels
#[derive(Debug)]
pub struct LBTChanConf {
    pub freq_hz: u32,
    pub scan_time_us: u16,
}

/// Configuration structure for LBT specificities
#[derive(Debug)]
pub struct LBTConf {
    /// enable LBT
    pub enable: bool,
    /// RSSI threshold to detect if channel is busy or not (dBm)
    pub rssi_target: i8,
    /// number of LBT channels
    pub nb_channel: u8,
    pub channels: [LBTChanConf; 8usize],
    /// RSSI offset to be applied to SX127x RSSI values
    pub rssi_offset: i8,
}

/// Configuration structure for a RF chain
#[derive(Debug)]
pub struct RxRFConf {
    /// enable or disable that RF chain
    pub enable: bool,
    /// center frequency of the radio in Hz
    pub freq_hz: u32,
    /// Board-specific RSSI correction factor
    pub rssi_offset: f32,
    /// Radio type for that RF chain (SX1255, SX1257....)
    pub type_: lgw_radio_type_e,
    /// enable or disable TX on that RF chain
    pub tx_enable: bool,
    pub tx_notch_freq: u32,
}

/// Configuration structure for an If chain
#[derive(Debug)]
pub struct RxIFConf {
    /// enable or disable that If chain
    pub enable: bool,
    /// to which RF chain is that If chain associated
    pub rf_chain: u8,
    /// center frequ of the If chain, relative to RF chain frequency
    pub freq_hz: i32,
    /// Rx bandwidth, 0 for default
    pub bandwidth: u8,
    /// Rx datarate, 0 for default
    pub datarate: u32,
    /// size of FSK sync word (number of bytes, 0 for default)
    pub sync_word_size: u8,
    pub sync_word: u64,
}

/// Structure containing the metadata of a packet that was received and a pointer to the payload
#[derive(Clone)]
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
    /// payload size in bytes
    pub size: u16,
    pub payload: [u8; 256usize],
}

/// Structure containing the configuration of a packet to send and a pointer to the payload
#[derive(Clone)]
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
    /// payload size in bytes
    pub size: u16,
    pub payload: [u8; 256usize],
}

/// Structure containing all gains of Tx chain
#[derive(Debug)]
pub struct TxGain {
    /// 2 bits, control of the digital gain of SX1301
    pub dig_gain: u8,
    /// 2 bits, control of the external PA (SX1301 I/O)
    pub pa_gain: u8,
    /// 2 bits, control of the radio DAC
    pub dac_gain: u8,
    /// 4 bits, control of the radio mixer
    pub mix_gain: u8,
    pub rf_power: i8,
}

/// Structure defining the Tx gain LUT
#[derive(Debug)]
pub struct TxGainLUT {
    /// Array of Tx gain struct
    pub lut: [TxGain; 16usize],
    pub size: u8,
}
