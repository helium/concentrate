#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl Default for lgw_pkt_rx_s {
    fn default() -> Self {
        Self {
            freq_hz: 0,
            if_chain: 0,
            status: 0,
            count_us: 0,
            rf_chain: 0,
            modulation: 0,
            bandwidth: 0,
            datarate: 0,
            coderate: 0,
            rssi: 0.0,
            snr: 0.0,
            snr_min: 0.0,
            snr_max: 0.0,
            crc: 0,
            size: 0,
            payload: [0u8; 256],
        }
    }
}
