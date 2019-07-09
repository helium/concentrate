extern crate messages;
use messages as msg;

#[macro_use]
pub mod macros;

#[derive(Debug, Copy, Clone)]
enum Quality {
    CrcOk,
    CrcFail,
    Missed,
}

pub struct LongFiPkt {
    oui: u32,
    device_id: u16,
    pub packet_id: u8,
    mac: u16,
    payload: Vec<u8>,
    num_fragments: u8,
    fragment_cnt: u8,
    quality: Vec<Quality>,
}

impl core::fmt::Debug for LongFiPkt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut quality = String::from("");

        for i in self.quality.iter() {
            match i {
                Quality::CrcOk => quality.push('O'),
                Quality::CrcFail => quality.push('S'),
                Quality::Missed => quality.push('X'),
            }
        }

        write!(
            f,
            "LongFiPkt {{ oui: 0x{:x}, device_id 0x{:x}, packet_id: 0x{:x}, mac: 0x{:x},
            quality: {},
            payload: {:?} }}",
            self.oui, self.device_id, self.packet_id, self.mac, quality, self.payload
        )
    }
}

const PAYLOAD_BEGIN_SINGLE_FRAGMENT_PACKET: usize = 9;
const PAYLOAD_BEGIN_MULTI_FRAGMENT_PACKET: usize = 11;
const PAYLOAD_BEGIN_FRAGMENT_PACKET: usize = 4;
pub struct LongFiParser {
    fragmented_packets: [Option<LongFiPkt>; 256],
}

pub enum ParserResponse {
    PKT(LongFiPkt),
    FRAGMENTED_PACKET_BEGIN(usize),
}

impl LongFiParser {
    pub fn new() -> LongFiParser {
        LongFiParser {
            fragmented_packets: array_of_none_256!(),
        }
    }

    pub fn timeout(&mut self, index: usize) -> Option<ParserResponse> {
        if let Some(mut pkt) = self.fragmented_packets[index].take() {
            while pkt.num_fragments > pkt.fragment_cnt {
                pkt.fragment_cnt += 1;
                pkt.quality.push(Quality::Missed);
            }

            return Some(ParserResponse::PKT(pkt));
        }
        return None;
    }

    pub fn parse(&mut self, msg: &msg::Resp) -> Option<ParserResponse> {
        if let Some(message) = &msg.kind {
            if let msg::Resp_oneof_kind::rx_packet(lorapkt) = &message {
                // means single frament packet header
                if lorapkt.payload[0] == 0 {
                    let len_copy = lorapkt
                        .payload
                        .len()
                        .checked_sub(PAYLOAD_BEGIN_SINGLE_FRAGMENT_PACKET)
                        .unwrap_or(0);
                    let mut payload: Vec<u8> = vec![0; len_copy];
                    payload.copy_from_slice(
                        &lorapkt.payload[PAYLOAD_BEGIN_SINGLE_FRAGMENT_PACKET
                            ..PAYLOAD_BEGIN_SINGLE_FRAGMENT_PACKET + len_copy],
                    );

                    let mut quality = Vec::new();
                    if lorapkt.crc_check {
                        quality.push(Quality::CrcOk);
                    } else {
                        quality.push(Quality::CrcFail);
                    }

                    return Some(ParserResponse::PKT(LongFiPkt {
                        packet_id: lorapkt.payload[0],
                        oui: (lorapkt.payload[1] as u32)
                            | (lorapkt.payload[2] as u32) << 8
                            | (lorapkt.payload[3] as u32) << 16
                            | (lorapkt.payload[4] as u32) << 24,
                        device_id: (lorapkt.payload[5] as u16) | (lorapkt.payload[6] as u16) << 8,
                        mac: (lorapkt.payload[7] as u16) | (lorapkt.payload[8] as u16) << 8,
                        payload,
                        num_fragments: 1,
                        fragment_cnt: 1,
                        quality,
                    }));
                }
                // means multi-fragment packet header
                else if lorapkt.payload[1] == 0 {
                    let len_copy = lorapkt
                        .payload
                        .len()
                        .checked_sub(PAYLOAD_BEGIN_MULTI_FRAGMENT_PACKET)
                        .unwrap_or(0);

                    let mut payload: Vec<u8> = vec![0; len_copy];
                    payload.copy_from_slice(
                        &lorapkt.payload[PAYLOAD_BEGIN_MULTI_FRAGMENT_PACKET
                            ..PAYLOAD_BEGIN_MULTI_FRAGMENT_PACKET + len_copy],
                    );

                    let packet_id = lorapkt.payload[0] as usize;

                    let mut quality = Vec::new();
                    if lorapkt.crc_check {
                        quality.push(Quality::CrcOk);
                    } else {
                        quality.push(Quality::CrcFail);
                    }

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
                            mac: (lorapkt.payload[9] as u16) | (lorapkt.payload[10] as u16) << 8,
                            payload,
                            quality,
                        }
                    });

                    return Some(ParserResponse::FRAGMENTED_PACKET_BEGIN(packet_id));
                }
                // must be fragment
                else {
                    let packet_id = lorapkt.payload[0] as usize;

                    let len_copy = lorapkt
                        .payload
                        .len()
                        .checked_sub(PAYLOAD_BEGIN_FRAGMENT_PACKET)
                        .unwrap_or(0);

                    // assert
                    let mut ret = false;

                    if let Some(pkt) = &mut self.fragmented_packets[packet_id] {
                        let fragment_num = lorapkt.payload[1];

                        while fragment_num > pkt.fragment_cnt {
                            pkt.fragment_cnt += 1;
                            pkt.quality.push(Quality::Missed);
                        }

                        if fragment_num == pkt.fragment_cnt {
                            if lorapkt.crc_check {
                                pkt.quality.push(Quality::CrcOk);
                            } else {
                                pkt.quality.push(Quality::CrcFail);
                            }

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
                        if let Some(pkt) = self.fragmented_packets[packet_id].take() {
                            return Some(ParserResponse::PKT(pkt));
                        }
                    }
                }
            }
        }
        None
    }
}
