extern crate messages;
use messages as msg;
use msg::LongFiSpreading as Spreading;
use lfc_sys;
use lfc_sys::lfc;
use lfc_sys::lfc_user_cfg as LfcUserCfg;
use lfc_sys::lfc_dg_des as LfcDatagram;
use lfc_sys::cursor as Cursor;
use lfc_sys::lfc_res as LfcResp;
use lfc_sys::lfc_dg_type as LfcDg;
use std::ffi::c_void;

struct LongFiCore {
    cb_data: usize,
    key: [u8; 16],
    c_handle: Option<lfc>
}

impl LongFiCore {
    pub fn new() -> LongFiCore {
        let mut ret = LongFiCore {
            c_handle: None,
            key: [0; 16],
            cb_data: 0,
        };
        ret.c_handle = Some( lfc {
           seq: 0,
           cfg: LfcUserCfg {
             cb_data: ret.cb_data as *mut c_void,
             key: ret.key[0] as *mut c_void,
             oui: 0x0,
             did: 0x1,
             key_len: 16,
           }
        });
        ret
    }

    pub fn parse(&self, pkt: &messages::RadioRxPacket) -> Option<LongFiPkt> {
        let mut output;
        unsafe {
            output = core::mem::zeroed::<LfcDatagram>();
        }

        let mut buf = [0u8; 1024];
        for i in (0..pkt.payload.len()) {
            buf[i] = pkt.payload[i];
        }

        let mut input = Cursor {
            buf: buf[0] as *mut u8,
            len: pkt.payload.len(),
            pos: pkt.payload.len(),
        };

        let response;
        unsafe {
            response = lfc_sys::lfc_dg__des(&mut output, &mut input);
        }

        let quality: Vec<Quality> = vec![
            match pkt.crc_check {
                true => Quality::CrcOk,
                false => Quality::CrcFail
            }
        ];

        match response {
            LfcResp::lfc_res_ok => {
                match output.type_ {
                    LfcDg::lfc_dg_type_monolithic => {
                        let monolithic = unsafe { output.__bindgen_anon_1.monolithic.as_ref() };
                        Some(LongFiPkt {
                                oui: monolithic.oui,//output.monolithic.oui,
                                device_id: monolithic.did,
                                packet_id: 0,
                                mac: monolithic.fp,
                                payload: buf.to_vec(),
                                num_fragments: 1,
                                fragment_cnt: 1,
                                timestamp: pkt.timestamp,
                                snr: pkt.snr,
                                rssi: pkt.rssi,
                                spreading: Spreading::SF10,
                                quality,
                                crc_fails: 0,
                        })
                    }
                    _ => None,
                }

            },
            LfcResp::lfc_res_err_exception => None,
            LfcResp::lfc_res_err_nomem => None,
            LfcResp::lfc_res_invalid_type => None,
            LfcResp::lfc_res_invalid_flags => None,
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
enum Quality {
    CrcOk,
    CrcFail,
    Missed,
}


pub struct LongFiPkt {
    oui: u32,
    device_id: u32,
    pub packet_id: u8,
    mac: u32,
    payload: Vec<u8>,
    num_fragments: u8,
    fragment_cnt: u8,
    timestamp: u64,
    snr: f32,
    rssi: f32,
    spreading: Spreading,
    quality: Vec<Quality>,
    crc_fails: usize,
}

impl LongFiPkt {
    pub fn quality_string(&self) -> String {
        let mut quality = String::new();

        for i in &self.quality {
            match i {
                Quality::CrcOk => quality.push('O'),
                Quality::CrcFail => quality.push('S'),
                Quality::Missed => quality.push('X'),
            }
        }

        quality
    }
}

use messages::LongFiRxPacket;

impl From<LongFiPkt> for LongFiRxPacket {
    fn from(other: LongFiPkt) -> LongFiRxPacket {
        let mut crc_check = true;

        for i in other.quality {
            if i != Quality::CrcOk {
                crc_check = false;
                break;
            }
        }

        LongFiRxPacket {
            crc_check,
            timestamp: other.timestamp,
            rssi: other.rssi,
            snr: other.snr,
            oui: other.oui as u32,
            device_id: u32::from(other.device_id),
            mac: u32::from(other.mac),
            payload: other.payload,
            spreading: other.spreading,
            // special fields
            unknown_fields: Default::default(),
            cached_size: Default::default(),
        }
    }
}

impl core::fmt::Debug for LongFiPkt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let quality = self.quality_string();

        write!(
            f,
            "LongFiPkt {{ oui: 0x{:x}, device_id 0x{:x}, packet_id: 0x{:x}, mac: 0x{:x}, quality: {}, crc_fails: {}, payload: {:?} }}",
            self.oui, self.device_id, self.packet_id, self.mac, quality, self.crc_fails, self.payload
        )
    }
}

impl PartialEq for LongFiPkt {
    fn eq(&self, other: &Self) -> bool {
        self.oui == other.oui && self.device_id == other.device_id && self.payload == other.payload
    }
}
