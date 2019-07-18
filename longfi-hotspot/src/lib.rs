extern crate messages;
use messages as msg;

#[macro_use]
pub mod macros;

#[derive(Debug, Copy, Clone, PartialEq)]
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

impl LongFiPkt {
    fn from_req(req: &messages::LongFiTxUplinkPacket) -> LongFiPkt {
        LongFiPkt {
            oui: req.oui,
            device_id: req.device_id as u16,
            packet_id: 0,
            mac: 0x00,
            payload: req.payload.to_vec(),
            num_fragments: 0,
            fragment_cnt: 0,
            quality: Default::default(),
        }
    }
}

use messages::LongFiRxPacket;

impl Into<LongFiRxPacket> for LongFiPkt {
    fn into(self) -> LongFiRxPacket {

        let mut crc_check = true;

        for i in self.quality.iter() {
            if i != &Quality::CrcOk {
                crc_check = false;
                break;
            }
        }

        LongFiRxPacket {
            crc_check,
            timestamp: 100,
            rssi: 1.0,
            snr: 1.0,
            oui: self.oui as u32,
            device_id: self.device_id as u32,
            mac: self.mac as u32,
            payload: self.payload,
            // special fields
            unknown_fields: Default::default(),
            cached_size: Default::default(),
        }
    }
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



impl PartialEq for LongFiPkt {
    fn eq(&self, other: &Self) -> bool {
        if (self.oui == other.oui) && (self.device_id == other.device_id) &&  (self.device_id == other.device_id) && (self.payload.len() == other.payload.len()){
            for (pos, e) in self.payload.iter().enumerate() {
                if *e != other.payload[pos] {
                    return false;
                }
            }
            return true;
        } else {
            return false;
        }
    }
}


const PAYLOAD_BEGIN_SINGLE_FRAGMENT_PACKET: usize = 9;
const PAYLOAD_BEGIN_MULTI_FRAGMENT_PACKET: usize = 11;
const PAYLOAD_BEGIN_FRAGMENT_PACKET: usize = 4;
pub struct LongFiParser {
    fragmented_packets: [Option<LongFiPkt>; 256],
}

pub enum LongFiResponse {
    Pkt(LongFiPkt),
    FragmentedPacketBegin(usize),
    SocketError,
    SentPacket(LongFiPkt),
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

            return Some(LongFiResponse::Pkt(pkt));
        }
        return None;
    }

    pub fn parse(&mut self, msg: &msg::Resp) -> Option<LongFiResponse> {
        if let Some(message) = &msg.kind {
            if let msg::Resp_oneof_kind::rx_packet(pkt) = &message {
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

                    return Some(LongFiResponse::Pkt(LongFiPkt {
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
                            return Some(LongFiResponse::Pkt(pkt));
                        }
                    }
                }
            } else {
                println!("did not parse");
            }
        } else {
            println!("did not parse!");
        }
        None
    }
}

extern crate mio;
pub struct LongFiSender;

extern crate protobuf;
use protobuf::{parse_from_bytes, Message};

fn msg_send<T: Message>(msg: T, socket: &mio::net::UdpSocket, addr_out: &std::net::SocketAddr) -> std::io::Result<()> {
    let mut enc_buf = Vec::new();
    msg.write_to_vec(&mut enc_buf)
        .expect("error serializing packet");
    socket.send_to(&enc_buf, addr_out)?;
    Ok(())
}

impl LongFiSender {
    pub fn new() -> LongFiSender {
        LongFiSender {}
    }

    pub fn send(&mut self, msg: &msg::Req, socket: &mio::net::UdpSocket, addr_out: &std::net::SocketAddr) -> Option<LongFiResponse> {
        if let Some(message) = &msg.kind {
            if let msg::Req_oneof_kind::longfi_tx(req) = &message {
                println!("Tx request");

                let mut longfi_payload = vec![
                    0x00,                       // uint8_t packet_id;  // 0    must be zero
                    0x00, 0x00, 0x00, 0x00,     // uint32_t oui;       // 1:4
                    0x00, 0x00,                 // uint16_t device_id; // 5:6
                    0x00, 0x00                  // uint16_t mac;       // 7:8
                ];

                longfi_payload.extend(&req.payload);
                println!("Sending {:?}", longfi_payload);

                let tx_req = msg::TxReq {
                        freq: 916_600_000,
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
                if let Err(e) = msg_send(
                    msg::Req {
                        id: 0xfe,
                        kind: Some(msg::Req_oneof_kind::tx(tx_req)),
                        ..Default::default()
                    },
                    &socket,
                    &addr_out,
                ) {
                    return Some(LongFiResponse::SocketError);
                } else {
                    return Some(LongFiResponse::SentPacket(LongFiPkt::from_req(req)));
                }

            }
        }

        None
    }

}