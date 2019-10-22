use super::LongFiResponse;
use crate::msg::LongFiSpreading as Spreading;
use messages as msg;
use rand::Rng;
use std::collections::VecDeque;
use lfc;

const SIZEOF_PACKET_HEADER: usize = std::mem::size_of::<PacketHeader>();
const SIZEOF_PACKET_HEADER_MULTIPLE_FRAGMENTS: usize =
    std::mem::size_of::<PacketHeaderMultipleFragments>();
const SIZEOF_FRAGMENT_HEADER: usize = std::mem::size_of::<FragmentHeader>();

#[repr(C, packed(1))]
// if first byte is 0, is single fragment packet_header
pub struct PacketHeader {
    pub packet_id: u8,  // 0    must be zero
    pub oui: u32,       // 1:4
    pub device_id: u16, // 5:6
    pub mac: u16,       // 7:8
}

impl PacketHeader {
    fn new(tx_uplink: &msg::LongFiTxUplinkPacket) -> PacketHeader {
        PacketHeader {
            packet_id: 0x00,
            oui: tx_uplink.oui,
            device_id: tx_uplink.device_id as u16,
            mac: 0x00,
        }
    }
}

impl From<PacketHeader> for [u8; SIZEOF_PACKET_HEADER] {
    fn from(other: PacketHeader) -> [u8; SIZEOF_PACKET_HEADER] {
        unsafe { std::mem::transmute::<PacketHeader, [u8; SIZEOF_PACKET_HEADER]>(other) }
    }
}

#[repr(C, packed(1))]
// if second byte is 0, is multi-fragment packet_header
pub struct PacketHeaderMultipleFragments {
    pub packet_id: u8,     // 0    must be non-zero
    pub fragment_num: u8,  // 1    must be zero (byte)
    pub num_fragments: u8, // 2    must be non-zero
    pub oui: u32,          // 3:6
    pub device_id: u16,    // 7:8
    pub mac: u16,          // 9:10
}

impl PacketHeaderMultipleFragments {
    fn new(
        tx_uplink: &msg::LongFiTxUplinkPacket,
        packet_id: u8,
        num_fragments: u8,
    ) -> PacketHeaderMultipleFragments {
        PacketHeaderMultipleFragments {
            packet_id,
            fragment_num: 0,
            num_fragments,
            oui: tx_uplink.oui,
            device_id: tx_uplink.device_id as u16,
            mac: 0x00,
        }
    }
}

impl From<PacketHeaderMultipleFragments> for [u8; SIZEOF_PACKET_HEADER_MULTIPLE_FRAGMENTS] {
    fn from(other: PacketHeaderMultipleFragments) -> [u8; SIZEOF_PACKET_HEADER_MULTIPLE_FRAGMENTS] {
        unsafe {
            std::mem::transmute::<
                PacketHeaderMultipleFragments,
                [u8; SIZEOF_PACKET_HEADER_MULTIPLE_FRAGMENTS],
            >(other)
        }
    }
}

#[repr(C, packed(1))]
// else (first and second byte, non-zero), is packet fragment
pub struct FragmentHeader {
    pub packet_id: u8,    // 0    must be non-zero
    pub fragment_num: u8, // 1    must be non-zero
    pub mac: u16,         // 2:3
}

impl FragmentHeader {
    fn new(packet_id: u8, fragment_num: u8) -> FragmentHeader {
        FragmentHeader {
            packet_id,
            fragment_num,
            mac: 0x00,
        }
    }
}

impl From<FragmentHeader> for [u8; SIZEOF_FRAGMENT_HEADER] {
    fn from(other: FragmentHeader) -> [u8; SIZEOF_FRAGMENT_HEADER] {
        unsafe { std::mem::transmute::<FragmentHeader, [u8; SIZEOF_FRAGMENT_HEADER]>(other) }
    }
}

pub struct LongFiSender {
    rng: rand::ThreadRng,
    req_id: Option<u32>,
    pending_fragments: Option<VecDeque<msg::RadioReq>>,
}

const RADIO_1: u32 = 920_600_000;
const RADIO_2: u32 = 916_600_000;
const FREQ_SPACING: u32 = 200_000;
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

fn payload_per_fragment(spreading: Spreading) -> usize {
    match spreading {
        Spreading::SF7 => 32,
        Spreading::SF8 => 32,
        Spreading::SF9 => 24,
        Spreading::SF10 => 24,
        Spreading::SF_INVALID => 0,
    }
}

// number of bytes in a fragment
fn payload_bytes_in_single_fragment_packet(spreading: Spreading) -> usize {
    payload_per_fragment(spreading) - SIZEOF_PACKET_HEADER
}

// number of bytes in a fragment
fn payload_bytes_in_first_fragment_of_many(spreading: Spreading) -> usize {
    payload_per_fragment(spreading) - SIZEOF_PACKET_HEADER_MULTIPLE_FRAGMENTS
}

// number of bytes in a fragment
fn payload_bytes_in_subsequent_fragments(spreading: Spreading) -> usize {
    payload_per_fragment(spreading) - SIZEOF_FRAGMENT_HEADER
}

impl LongFiSender {
    pub fn new() -> LongFiSender {
        LongFiSender {
            rng: rand::thread_rng(),
            req_id: None,
            pending_fragments: None,
        }
    }

    pub fn new_fragment(&mut self, spreading: Spreading, payload: Vec<u8>) -> msg::RadioReq {
        msg::RadioReq {
            id: 0xfe,
            kind: Some(msg::RadioReq_oneof_kind::tx(msg::RadioTxReq {
                freq: CHANNEL[self.rng.gen::<usize>() % LONGFI_NUM_UPLINK_CHANNELS],
                radio: msg::Radio::R0,
                power: 28,
                bandwidth: msg::Bandwidth::BW125kHz,
                spreading: spreading.into(),
                coderate: msg::Coderate::CR4_5,
                invert_polarity: false,
                omit_crc: false,
                implicit_header: false,
                payload,
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    pub fn tx_resp(&mut self) -> Option<LongFiResponse> {
        debug!("[LongFi] Radio has signalled that packet was sent");

        let mut clear_pending_fragments = false;

        let ret = match &mut self.pending_fragments {
            // if there is a vector, we should have more fragments
            Some(vec) => {
                debug!("[LongFi] Sending another fragment. {} remaining", vec.len());
                let maybe_fragment = vec.pop_front();

                if vec.is_empty() {
                    clear_pending_fragments = true;
                }

                match maybe_fragment {
                    Some(fragment) => {
                        // print the fragment payload nicely
                        if let Some(msg) = &fragment.kind {
                            let msg::RadioReq_oneof_kind::tx(tx) = msg;
                            debug!("[LongFi] Fragment: {:?}", tx.payload);
                        }

                        Some(LongFiResponse::RadioReq(fragment))
                    }
                    None => None,
                }
            }
            // if None, just completed a full packet
            None => match self.req_id.take() {
                Some(id) => {
                    debug!("[LongFi] All packet fragments transmitted");
                    Some(LongFiResponse::ClientResp(msg::LongFiResp {
                        id,
                        kind: Some(msg::LongFiResp_oneof_kind::tx_status(msg::LongFiTxStatus {
                            success: true,
                            ..Default::default()
                        })),
                        ..Default::default()
                    }))
                }
                None => None,
            },
        };

        if clear_pending_fragments {
            self.pending_fragments = None;
        }
        ret
    }

    pub fn tx_uplink(&mut self, tx_uplink: &msg::LongFiTxUplinkPacket) -> Option<LongFiResponse> {
        if let Some(pkt) = lfc::serialize(&mut self.rng, tx_uplink) {
            Some(LongFiResponse::RadioReq(pkt))
        } else {
            None
        }
    }
}
