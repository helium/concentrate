extern crate messages;
use lfc_sys;
use lfc_sys::cursor as Cursor;
use lfc_sys::lfc_dg_des as LfcDatagram;
use lfc_sys::lfc_dg_monolithic as MonolithicDg;
use lfc_sys::lfc_dg_monolithic_lfc_dg_monolithic_flags as DgFlags;
use lfc_sys::lfc_dg_type as LfcDg;
use lfc_sys::lfc_res as LfcResp;
use messages as msg;
use msg::LongFiSpreading as Spreading;
use rand;
use rand::Rng;

const ONION_OUI: u32 = 0;
const ONION_DID: u32 = 1;

const RADIO_1: u32 = 920_600_000;
const RADIO_2: u32 = 916_600_000;
const FREQ_SPACING: u32 = 200_000;
const LONGFI_NUM_UPLINK_CHANNELS: usize = 8;

const CHANNEL: [u32; LONGFI_NUM_UPLINK_CHANNELS as usize] = [
    RADIO_1 - FREQ_SPACING * 2,
    RADIO_1 - FREQ_SPACING,
    RADIO_1,
    RADIO_2 - FREQ_SPACING * 2,
    RADIO_2 - FREQ_SPACING,
    RADIO_2,
    RADIO_2 + FREQ_SPACING,
    RADIO_2 + FREQ_SPACING * 2,
];

pub fn parse(pkt: &messages::RadioRxPacket) -> Option<LongFiPkt> {
    let mut output;
    unsafe {
        output = core::mem::zeroed::<LfcDatagram>();
    }

    let mut buf = [0u8; 1024];
    let len = pkt.payload.len();

    for (idx, element) in pkt.payload.iter().enumerate() {
        buf[idx] = *element;
    }

    let mut input = Cursor {
        buf: buf.as_mut_ptr() as *mut u8,
        len,
        pos: 0,
    };

    let response;
    unsafe {
        response = lfc_sys::lfc_dg__des(&mut output, &mut input);
    }

    let quality: Vec<Quality> = vec![if pkt.crc_check {
        Quality::CrcOk
    } else {
        Quality::CrcFail
    }];

    match response {
        LfcResp::lfc_res_ok => match output.type_ {
            LfcDg::lfc_dg_type_monolithic => {
                let monolithic = unsafe { output.__bindgen_anon_1.monolithic.as_ref() };

                let mut payload = monolithic.pay.to_vec();
                payload.resize(monolithic.pay_len, 0);

                Some(LongFiPkt {
                    oui: monolithic.oui,
                    device_id: monolithic.did,
                    packet_id: 0,
                    fingerprint: monolithic.fp,
                    payload,
                    timestamp: pkt.timestamp,
                    snr: pkt.snr,
                    rssi: pkt.rssi,
                    spreading: Spreading::SF10,
                    quality,
                    crc_fails: 0,
                })
            }
            LfcDg::lfc_dg_type_frame_start => {
                //error!("Frame Start");
                None
            }
            LfcDg::lfc_dg_type_frame_data => {
                //error!("Frame Data");
                None
            }
            LfcDg::lfc_dg_type_ack => {
                //error!("ACK");
                None
            }
        },
        LfcResp::lfc_res_err_exception => {
            //error!("Generic, exceptional error");
            None
        }
        LfcResp::lfc_res_err_nomem => {
            //error!("Provided buffer is too small for request.");
            None
        }
        LfcResp::lfc_res_invalid_type => {
            //error!("Invalid datagram type");
            None
        }
        LfcResp::lfc_res_invalid_flags => {
            //error!("Invalid datagram flags.");
            None
        }
    }
}

pub fn serialize(rng: &mut rand::ThreadRng, pkt: &msg::LongFiTxPacket) -> Option<msg::RadioReq> {
    let mut input = unsafe { core::mem::zeroed::<MonolithicDg>() };

    input.flags = DgFlags {
        downlink: pkt.downlink,
        should_ack: pkt.should_ack,
        cts_rts: pkt.cts,
        priority: pkt.priority,
        ldpc: pkt.ldpc,
    };

    input.oui = ONION_OUI;
    input.did = ONION_DID;
    input.fp = 0x0;
    input.seq = 0x0;
    input.pay_len = pkt.payload.len();

    for (idx, element) in pkt.payload.iter().enumerate() {
        input.pay[idx] = *element;
    }

    let mut buf = [0u8; 1024];
    let mut cursor = Cursor {
        buf: buf.as_mut_ptr() as *mut u8,
        len: 1024,
        pos: 0,
    };

    let response;
    unsafe {
        response = lfc_sys::lfc_dg_monolithic__ser(&input, &mut cursor);
    }

    match response {
        LfcResp::lfc_res_ok => Some(msg::RadioReq {
            id: 0xfe,
            kind: Some(msg::RadioReq_oneof_kind::tx(msg::RadioTxReq {
                freq: CHANNEL[rng.gen::<usize>() % LONGFI_NUM_UPLINK_CHANNELS],
                radio: msg::Radio::R0,
                power: 28,
                bandwidth: msg::Bandwidth::BW125kHz,
                spreading: msg::Spreading::SF10,
                coderate: msg::Coderate::CR4_5,
                invert_polarity: false,
                omit_crc: false,
                implicit_header: false,
                payload: buf.to_vec(),
                ..Default::default()
            })),
            ..Default::default()
        }),
        _ => None,
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Quality {
    CrcOk,
    CrcFail,
}

pub struct LongFiPkt {
    oui: u32,
    device_id: u32,
    pub packet_id: u8,
    fingerprint: u32,
    payload: Vec<u8>,
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
            device_id: other.device_id,
            fingerprint: other.fingerprint,
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
            "LongFiPkt {{ oui: 0x{:x}, device_id 0x{:x}, packet_id: 0x{:x}, fingerprint: 0x{:x}, quality: {}, crc_fails: {}, payload: {:?} }}",
            self.oui, self.device_id, self.packet_id, self.fingerprint, quality, self.crc_fails, self.payload
        )
    }
}

impl PartialEq for LongFiPkt {
    fn eq(&self, other: &Self) -> bool {
        self.oui == other.oui && self.device_id == other.device_id && self.payload == other.payload
    }
}
