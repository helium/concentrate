use crate::error::AppResult;
use messages as msg;
use protobuf::parse_from_bytes;
use std::net::{SocketAddr, UdpSocket};

struct LongFiPkt {
    oui: u32,
    device_id: u16,
    packet_id: u8,
    mac: u16,
    payload: Vec<u8>,
    num_fragments: u8,
    fragment_cnt: u8,
}

impl core::fmt::Debug for LongFiPkt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "LongFiPkt {{ oui: 0x{:x}, device_id 0x{:x}, packet_id: 0x{:x}, mac: 0x{:x},
            payload: {:?} }}",
            self.oui, self.device_id, self.packet_id, self.mac, self.payload
        )
    }
}

use byteorder::{ByteOrder, LittleEndian};

const PAYLOAD_BEGIN_SINGLE_FRAGMENT_PACKET: usize = 9;
const PAYLOAD_BEGIN_MULTI_FRAGMENT_PACKET: usize = 11;
const PAYLOAD_BEGIN_FRAGMENT_PACKET: usize = 4;
struct LongFiParser {
    fragmented_packets: [Option<LongFiPkt>; 256],
}

macro_rules! array_of_none_256 {
    () => {
        [
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None,
        ];
    };
}

impl LongFiParser {
    fn new() -> LongFiParser {
        LongFiParser {
            fragmented_packets: array_of_none_256!(),
        }
    }

    fn parse(&mut self, msg: &msg::Resp) -> Option<LongFiPkt> {
        if let Some(message) = &msg.kind {
            if let msg::Resp_oneof_kind::rx_packet(lorapkt) = &message {
                if lorapkt.crc_check {
                    // means single frament packet header
                    if lorapkt.payload[0] == 0 {
                        let len_copy = lorapkt.payload.len() - PAYLOAD_BEGIN_SINGLE_FRAGMENT_PACKET;
                        let mut payload: Vec<u8> = vec![0; len_copy];
                        payload.copy_from_slice(
                            &lorapkt.payload[PAYLOAD_BEGIN_SINGLE_FRAGMENT_PACKET
                                ..PAYLOAD_BEGIN_SINGLE_FRAGMENT_PACKET + len_copy],
                        );

                        return Some(LongFiPkt {
                            packet_id: lorapkt.payload[0],
                            oui: (lorapkt.payload[1] as u32)
                                | (lorapkt.payload[2] as u32) << 8
                                | (lorapkt.payload[3] as u32) << 16
                                | (lorapkt.payload[4] as u32) << 24,
                            device_id: (lorapkt.payload[5] as u16)
                                | (lorapkt.payload[6] as u16) << 8,
                            mac: (lorapkt.payload[7] as u16) | (lorapkt.payload[8] as u16) << 8,
                            payload,
                            num_fragments: 1,
                            fragment_cnt: 1,
                        });
                    }
                    // means multi-fragment packet header
                    else if lorapkt.payload[1] == 0 {
                        let len_copy = lorapkt.payload.len() - PAYLOAD_BEGIN_MULTI_FRAGMENT_PACKET;
                        let mut payload: Vec<u8> = vec![0; len_copy];
                        payload.copy_from_slice(
                            &lorapkt.payload[PAYLOAD_BEGIN_MULTI_FRAGMENT_PACKET
                                ..PAYLOAD_BEGIN_MULTI_FRAGMENT_PACKET + len_copy],
                        );

                        let packet_id = lorapkt.payload[0] as usize;

                        // let len_copy = lorapkt.payload.len() - PAYLOAD_BEGIN;
                        // assert

                        if !self.fragmented_packets[packet_id].is_some() {
                            self.fragmented_packets[packet_id] = Some({
                                LongFiPkt {
                                    packet_id: packet_id as u8,
                                    num_fragments: lorapkt.payload[2],
                                    fragment_cnt: 1,
                                    oui: (lorapkt.payload[3] as u32)
                                        | (lorapkt.payload[4] as u32) << 8
                                        | (lorapkt.payload[5] as u32) << 16
                                        | (lorapkt.payload[6] as u32) << 24,
                                    device_id: (lorapkt.payload[7] as u16)
                                        | (lorapkt.payload[8] as u16) << 8,
                                    mac: (lorapkt.payload[9] as u16)
                                        | (lorapkt.payload[10] as u16) << 8,
                                    payload,
                                }
                            });
                        }
                    }
                    // must be fragment
                    else {
                        let packet_id = lorapkt.payload[0] as usize;

                        let len_copy = lorapkt.payload.len() - PAYLOAD_BEGIN_FRAGMENT_PACKET;
                        // let len_copy = lorapkt.payload.len() - PAYLOAD_BEGIN;
                        // assert
                        let mut ret = false;

                        if let Some(pkt) = &mut self.fragmented_packets[packet_id] {
                            let fragment_num = lorapkt.payload[1];
                            if fragment_num == pkt.fragment_cnt {
                                pkt.payload.extend(
                                    lorapkt.payload[PAYLOAD_BEGIN_FRAGMENT_PACKET
                                        ..PAYLOAD_BEGIN_FRAGMENT_PACKET + len_copy]
                                        .iter()
                                        .cloned(),
                                );
                                pkt.fragment_cnt += 1;
                            }
                            if pkt.fragment_cnt == pkt.num_fragments {
                                ret = true;
                            }
                        }

                        if ret {
                            return self.fragmented_packets[packet_id].take();
                        }
                    }
                }
            }
        }
        None
    }
}

struct Router;
impl Router {
    pub fn receive_msg() {}
}

pub fn longfi(print_level: u8, resp_port: u16) -> AppResult {
    let resp_addr = SocketAddr::from(([0, 0, 0, 0], resp_port));

    debug!("listening for responses on {}", resp_addr);
    let socket = UdpSocket::bind(resp_addr)?;

    let mut read_buf = [0; 1024];

    let mut longfi = LongFiParser::new();

    loop {
        let (sz, src) = socket.recv_from(&mut read_buf)?;
        match parse_from_bytes::<msg::Resp>(&read_buf[..sz]) {
            Ok(rx_pkt) => {
                if let Some(pkt) = longfi.parse(&rx_pkt) {
                    super::print_at_level(print_level, &pkt);
                }
            }
            Err(e) => error!("{:?}", e),
        }
    }
}
