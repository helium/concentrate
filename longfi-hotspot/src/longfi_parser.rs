use super::{LongFiPkt, LongFiResponse, Quality};
use messages as msg;

const PAYLOAD_BEGIN_SINGLE_FRAGMENT_PACKET: usize = 9;
const PAYLOAD_BEGIN_MULTI_FRAGMENT_PACKET: usize = 11;
const PAYLOAD_BEGIN_FRAGMENT_PACKET: usize = 4;
pub struct LongFiParser {
    fragmented_packets: [Option<LongFiPkt>; 256],
}

impl LongFiParser {
    pub fn new() -> LongFiParser {
        LongFiParser {
            fragmented_packets: array_of_none_256!(),
        }
    }

    pub fn timeout(&mut self, index: usize) -> Option<LongFiResponse> {
        if let Some(mut pkt) = self.fragmented_packets[index].take() {
            while pkt.num_fragments > pkt.fragment_cnt {
                pkt.fragment_cnt += 1;
                pkt.quality.push(Quality::Missed);
            }

            return Some(LongFiResponse::PktRx(pkt));
        }
        return None;
    }

    pub fn parse(&mut self, pkt: &messages::RadioRxPacket) -> Option<LongFiResponse> {
        // if payload is smaller than the smallest header, it's invalid
        if pkt.payload.len() < PAYLOAD_BEGIN_FRAGMENT_PACKET {
            return None;
        }

        // means single frament packet header
        if pkt.payload[0] == 0 {
            if pkt.payload.len() < PAYLOAD_BEGIN_SINGLE_FRAGMENT_PACKET {
                return None;
            }
            let len_copy = pkt.payload.len() - PAYLOAD_BEGIN_SINGLE_FRAGMENT_PACKET;

            let mut payload: Vec<u8> = vec![0; len_copy];
            payload.copy_from_slice(
                &pkt.payload[PAYLOAD_BEGIN_SINGLE_FRAGMENT_PACKET
                    ..PAYLOAD_BEGIN_SINGLE_FRAGMENT_PACKET + len_copy],
            );

            let mut quality = Vec::new();
            if pkt.crc_check {
                quality.push(Quality::CrcOk);
            } else {
                quality.push(Quality::CrcFail);
            }

            return Some(LongFiResponse::PktRx(LongFiPkt {
                packet_id: pkt.payload[0],
                oui: (pkt.payload[1] as u32)
                    | (pkt.payload[2] as u32) << 8
                    | (pkt.payload[3] as u32) << 16
                    | (pkt.payload[4] as u32) << 24,
                device_id: (pkt.payload[5] as u16) | (pkt.payload[6] as u16) << 8,
                mac: (pkt.payload[7] as u16) | (pkt.payload[8] as u16) << 8,
                payload,
                num_fragments: 1,
                fragment_cnt: 1,
                quality,
                timestamp: pkt.timestamp,
                snr: pkt.snr,
                rssi: pkt.rssi,
            }));
        }
        // means multi-fragment packet header
        else if pkt.payload[1] == 0 {
            if pkt.payload.len() < PAYLOAD_BEGIN_MULTI_FRAGMENT_PACKET {
                return None;
            }
            let len_copy = pkt.payload.len() - PAYLOAD_BEGIN_MULTI_FRAGMENT_PACKET;

            let mut payload: Vec<u8> = vec![0; len_copy];
            payload.copy_from_slice(
                &pkt.payload[PAYLOAD_BEGIN_MULTI_FRAGMENT_PACKET
                    ..PAYLOAD_BEGIN_MULTI_FRAGMENT_PACKET + len_copy],
            );

            let packet_id = pkt.payload[0] as usize;

            let mut quality = Vec::new();
            if pkt.crc_check {
                quality.push(Quality::CrcOk);
            } else {
                quality.push(Quality::CrcFail);
            }

            self.fragmented_packets[packet_id] = Some({
                LongFiPkt {
                    packet_id: packet_id as u8,
                    num_fragments: pkt.payload[2],
                    fragment_cnt: 1,
                    oui: (pkt.payload[3] as u32)
                        | (pkt.payload[4] as u32) << 8
                        | (pkt.payload[5] as u32) << 16
                        | (pkt.payload[6] as u32) << 24,
                    device_id: (pkt.payload[7] as u16) | (pkt.payload[8] as u16) << 8,
                    mac: (pkt.payload[9] as u16) | (pkt.payload[10] as u16) << 8,
                    payload,
                    quality,
                    timestamp: 0,
                    snr: 0.0,
                    rssi: 0.0,
                }
            });

            return Some(LongFiResponse::FragmentedPacketBegin(packet_id));
        }
        // must be fragment
        else {
            let packet_id = pkt.payload[0] as usize;
            // we already know that the payload is at least the size of a fragment header
            let len_copy = pkt.payload.len() - PAYLOAD_BEGIN_FRAGMENT_PACKET;

            let mut ret = false;
            if let Some(fragmented_pkt) = &mut self.fragmented_packets[packet_id] {
                let fragment_num = pkt.payload[1];

                while fragment_num > fragmented_pkt.fragment_cnt {
                    fragmented_pkt.fragment_cnt += 1;
                    fragmented_pkt.quality.push(Quality::Missed);
                }

                if fragment_num == fragmented_pkt.fragment_cnt {
                    if pkt.crc_check {
                        fragmented_pkt.quality.push(Quality::CrcOk);
                    } else {
                        fragmented_pkt.quality.push(Quality::CrcFail);
                    }

                    fragmented_pkt.payload.extend(
                        pkt.payload[PAYLOAD_BEGIN_FRAGMENT_PACKET
                            ..PAYLOAD_BEGIN_FRAGMENT_PACKET + len_copy]
                            .iter()
                            .cloned(),
                    );
                    fragmented_pkt.fragment_cnt += 1;
                }

                if fragmented_pkt.fragment_cnt == fragmented_pkt.num_fragments {
                    ret = true;
                }
            }

            if ret {
                if let Some(pkt) = self.fragmented_packets[packet_id].take() {
                    return Some(LongFiResponse::PktRx(pkt));
                }
            }
            None
        }
    }
}
