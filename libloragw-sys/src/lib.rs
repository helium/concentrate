#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{fmt, slice, time};

include!("bindings.rs");

// We need to manually `impl Debug` due the non-`Debug` 256 byte array
// in the following structs.

impl fmt::Debug for lgw_pkt_rx_s {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("lgw_pkt_tx_s")
            .field("freq_hz", &self.freq_hz)
            .field("if_chain", &self.if_chain)
            .field("status", &self.status)
            .field("count_us", &self.count_us)
            .field("rf_chain", &self.rf_chain)
            .field("modulation", &self.modulation)
            .field("bandwidth", &self.bandwidth)
            .field("datarate", &self.datarate)
            .field("coderate", &self.coderate)
            .field("rssi", &self.rssi)
            .field("snr", &self.snr)
            .field("snr_min", &self.snr_min)
            .field("snr_max", &self.snr_max)
            .field("crc", &self.crc)
            .field("size", &self.size)
            .field("payload", unsafe {
                &slice::from_raw_parts(&self.payload as *const u8, self.size as usize)
            })
            .finish()
    }
}

impl fmt::Debug for lgw_pkt_tx_s {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("lgw_pkt_tx_s")
            .field("freq_hz", &self.freq_hz)
            .field("tx_mode", &self.tx_mode)
            .field("count_us", &self.count_us)
            .field("rf_chain", &self.rf_chain)
            .field("rf_power", &self.rf_power)
            .field("modulation", &self.modulation)
            .field("bandwidth", &self.bandwidth)
            .field("datarate", &self.datarate)
            .field("coderate", &self.coderate)
            .field("invert_pol", &self.invert_pol)
            .field("f_dev", &self.f_dev)
            .field("preamble", &self.preamble)
            .field("no_crc", &self.no_crc)
            .field("no_header", &self.no_header)
            .field("size", &self.size)
            .field("payload", unsafe {
                &slice::from_raw_parts(&self.payload as *const u8, self.size as usize)
            })
            .finish()
    }
}

impl From<timespec> for time::Duration {
    fn from(other: timespec) -> Self {
        Self::from_secs(other.tv_sec as u64) + Self::from_nanos(other.tv_nsec as u64)
    }
}

impl From<timespec> for time::SystemTime {
    fn from(other: timespec) -> Self {
        time::UNIX_EPOCH + time::Duration::from(other)
    }
}
