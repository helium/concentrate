use super::{LongFiPkt, LongFiResponse};
use messages as msg;
use protobuf::{parse_from_bytes, Message};
use rand::Rng;
use msg::LongFiSpreading as Spreading;

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

const SIZEOF_PACKET_HEADER: usize = 11;
const SIZEOF_PACKET_HEADER_MULTIPLE_FRAGMENTS: usize = 10;
const SIZEOF_FRAGMENT_HEADER: usize = 4;

fn payload_per_fragment(spreading: Spreading) -> usize {
    match spreading {
        Spreading::SF7 => 32,
        Spreading::SF8 => 32,
        Spreading::SF9 => 32,
        Spreading::SF10 => 32,
        Spreading::SF_INVALID => 0,
    }
}

// number of bytes in a fragment
fn payload_bytes_in_single_fragment_packet(spreading: Spreading) ->  usize {
  payload_per_fragment(spreading) - SIZEOF_PACKET_HEADER
}

// number of bytes in a fragment
fn payload_bytes_in_first_fragment_of_many(spreading: Spreading) ->  usize {
  payload_per_fragment(spreading) - SIZEOF_PACKET_HEADER_MULTIPLE_FRAGMENTS
}

// number of bytes in a fragment
fn payload_bytes_in_subsequent_fragments(spreading: Spreading) ->  usize {
  payload_per_fragment(spreading) - SIZEOF_FRAGMENT_HEADER
}

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
                kind: Some(msg::LongFiResp_oneof_kind::tx_status(msg::LongFiTxStatus {
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
            let mut num_fragments;
            let payload_consumed = 0;
            let packet_id = 0;
            //let num_bytes_copy;
            let len = tx_uplink.payload.len();

            if len < payload_bytes_in_single_fragment_packet(tx_uplink.spreading) {
                num_fragments = 1;
            } else {
                let remaining_len = len - payload_bytes_in_first_fragment_of_many(tx_uplink.spreading);
                num_fragments = 1 + remaining_len / payload_bytes_in_subsequent_fragments(tx_uplink.spreading);

                // if there was remainder, we need a final fragment
                if (remaining_len%payload_bytes_in_subsequent_fragments(tx_uplink.spreading) != 0){
                  num_fragments += 1;
                }
            }

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
