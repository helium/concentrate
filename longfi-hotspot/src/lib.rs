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
    Pkt(LongFiPkt),
    FragmentedPacketBegin(usize),
}

impl LongFiParser {
    pub fn new() -> LongFiParser {
        LongFiParser {
            fragmented_packets: array_of_none_256!(),
        }
    }

    pub fn timeout(&mut self, index: usize) -> Option<ParserResponse> {
        if let Some(mut Pkt) = self.fragmented_packets[index].take() {
            while Pkt.num_fragments > Pkt.fragment_cnt {
                Pkt.fragment_cnt += 1;
                Pkt.quality.push(Quality::Missed);
            }

            return Some(ParserResponse::Pkt(Pkt));
        }
        return None;
    }

    pub fn parse(&mut self, msg: &msg::Resp) -> Option<ParserResponse> {
        if let Some(message) = &msg.kind {
            if let msg::Resp_oneof_kind::rx_packet(loraPkt) = &message {
                // means single frament packet header
                if loraPkt.payload[0] == 0 {
                    let len_copy = loraPkt
                        .payload
                        .len()
                        .checked_sub(PAYLOAD_BEGIN_SINGLE_FRAGMENT_PACKET)
                        .unwrap_or(0);
                    let mut payload: Vec<u8> = vec![0; len_copy];
                    payload.copy_from_slice(
                        &loraPkt.payload[PAYLOAD_BEGIN_SINGLE_FRAGMENT_PACKET
                            ..PAYLOAD_BEGIN_SINGLE_FRAGMENT_PACKET + len_copy],
                    );

                    let mut quality = Vec::new();
                    if loraPkt.crc_check {
                        quality.push(Quality::CrcOk);
                    } else {
                        quality.push(Quality::CrcFail);
                    }

                    return Some(ParserResponse::Pkt(LongFiPkt {
                        packet_id: loraPkt.payload[0],
                        oui: (loraPkt.payload[1] as u32)
                            | (loraPkt.payload[2] as u32) << 8
                            | (loraPkt.payload[3] as u32) << 16
                            | (loraPkt.payload[4] as u32) << 24,
                        device_id: (loraPkt.payload[5] as u16) | (loraPkt.payload[6] as u16) << 8,
                        mac: (loraPkt.payload[7] as u16) | (loraPkt.payload[8] as u16) << 8,
                        payload,
                        num_fragments: 1,
                        fragment_cnt: 1,
                        quality,
                    }));
                }
                // means multi-fragment packet header
                else if loraPkt.payload[1] == 0 {
                    let len_copy = loraPkt
                        .payload
                        .len()
                        .checked_sub(PAYLOAD_BEGIN_MULTI_FRAGMENT_PACKET)
                        .unwrap_or(0);

                    let mut payload: Vec<u8> = vec![0; len_copy];
                    payload.copy_from_slice(
                        &loraPkt.payload[PAYLOAD_BEGIN_MULTI_FRAGMENT_PACKET
                            ..PAYLOAD_BEGIN_MULTI_FRAGMENT_PACKET + len_copy],
                    );

                    let packet_id = loraPkt.payload[0] as usize;

                    let mut quality = Vec::new();
                    if loraPkt.crc_check {
                        quality.push(Quality::CrcOk);
                    } else {
                        quality.push(Quality::CrcFail);
                    }

                    self.fragmented_packets[packet_id] = Some({
                        LongFiPkt {
                            packet_id: packet_id as u8,
                            num_fragments: loraPkt.payload[2],
                            fragment_cnt: 1,
                            oui: (loraPkt.payload[3] as u32)
                                | (loraPkt.payload[4] as u32) << 8
                                | (loraPkt.payload[5] as u32) << 16
                                | (loraPkt.payload[6] as u32) << 24,
                            device_id: (loraPkt.payload[7] as u16)
                                | (loraPkt.payload[8] as u16) << 8,
                            mac: (loraPkt.payload[9] as u16) | (loraPkt.payload[10] as u16) << 8,
                            payload,
                            quality,
                        }
                    });

                    return Some(ParserResponse::FragmentedPacketBegin(packet_id));
                }
                // must be fragment
                else {
                    let packet_id = loraPkt.payload[0] as usize;

                    let len_copy = loraPkt
                        .payload
                        .len()
                        .checked_sub(PAYLOAD_BEGIN_FRAGMENT_PACKET)
                        .unwrap_or(0);

                    // assert
                    let mut ret = false;

                    if let Some(Pkt) = &mut self.fragmented_packets[packet_id] {
                        let fragment_num = loraPkt.payload[1];

                        while fragment_num > Pkt.fragment_cnt {
                            Pkt.fragment_cnt += 1;
                            Pkt.quality.push(Quality::Missed);
                        }

                        if fragment_num == Pkt.fragment_cnt {
                            if loraPkt.crc_check {
                                Pkt.quality.push(Quality::CrcOk);
                            } else {
                                Pkt.quality.push(Quality::CrcFail);
                            }

                            Pkt.payload.extend(
                                loraPkt.payload[PAYLOAD_BEGIN_FRAGMENT_PACKET
                                    ..PAYLOAD_BEGIN_FRAGMENT_PACKET + len_copy]
                                    .iter()
                                    .cloned(),
                            );
                            Pkt.fragment_cnt += 1;
                        }

                        if Pkt.fragment_cnt == Pkt.num_fragments {
                            ret = true;
                        }
                    }

                    if ret {
                        if let Some(Pkt) = self.fragmented_packets[packet_id].take() {
                            return Some(ParserResponse::Pkt(Pkt));
                        }
                    }
                }
            }
        }
        None
    }
}
