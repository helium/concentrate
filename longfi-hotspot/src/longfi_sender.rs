use super::{LongFiPkt, LongFiResponse};
use messages as msg;
use protobuf::{parse_from_bytes, Message};
use rand::Rng;

pub struct LongFiSender {
    rng: rand::ThreadRng,
    req_id: Option<u32>,
}

const RADIO_1: u32 = 920600000;
const RADIO_2: u32 = 916600000;
const FREQ_SPACING: u32 = 200000;
const LONGFI_NUM_UPLINK_CHANNELS: usize = 8;

const CHANNEL: [u32; LONGFI_NUM_UPLINK_CHANNELS] = [
    RADIO_1 - FREQ_SPACING * 2,
    RADIO_1 - FREQ_SPACING,
    RADIO_1,
    RADIO_2 - FREQ_SPACING * 2,
    RADIO_2 - FREQ_SPACING,
    RADIO_2,
    RADIO_2 + FREQ_SPACING,
    RADIO_2 + FREQ_SPACING * 2,
];

impl LongFiSender {
    pub fn new() -> LongFiSender {
        LongFiSender {
            rng: rand::thread_rng(),
            req_id: None,
        }
    }

    pub fn tx_resp(&mut self, tx_resp: &msg::RadioTxResp) -> Option<LongFiResponse> {
        match self.req_id.take() {
            Some(id) => Some(LongFiResponse::ClientResp(msg::LongFiResp {
                id,
                kind: Some(msg::LongFiResp_oneof_kind::tx(msg::LongFiTxResp {
                    success: true,
                    ..Default::default()
                })),
                ..Default::default()
            })),
            None => None,
        }
    }

    pub fn tx_uplink(
        &mut self,
        tx_uplink: &msg::LongFiTxUplinkPacket,
        id: u32,
    ) -> Option<LongFiResponse> {
        match self.req_id {
            Some(id) => None, // should throw error
            None => {
                self.req_id = Some(id);
                let mut longfi_payload = vec![
                    0x00,
                    tx_uplink.oui as u8,
                    (tx_uplink.oui >> 8) as u8,
                    (tx_uplink.oui >> 16) as u8,
                    (tx_uplink.oui >> 24) as u8,
                    tx_uplink.device_id as u8,
                    (tx_uplink.device_id >> 8) as u8,
                    0x00,
                    0x00, // uint16_t mac;       // 7:8
                ];
                longfi_payload.extend(&tx_uplink.payload);

                let tx_req = msg::RadioTxReq {
                    freq: CHANNEL[self.rng.gen::<usize>() % LONGFI_NUM_UPLINK_CHANNELS],
                    radio: msg::Radio::R0,
                    power: 22,
                    bandwidth: msg::Bandwidth::BW125kHz,
                    spreading: msg::Spreading::SF9,
                    coderate: msg::Coderate::CR4_5,
                    invert_polarity: false,
                    omit_crc: false,
                    implicit_header: false,
                    payload: longfi_payload,
                    ..Default::default()
                };

                Some(LongFiResponse::RadioReq(msg::RadioReq {
                    id: 0xfe,
                    kind: Some(msg::RadioReq_oneof_kind::tx(tx_req)),
                    ..Default::default()
                }))
            }
        }
    }
}
