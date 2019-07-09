use crate::error::AppResult;
use messages as msg;
use protobuf::parse_from_bytes;
use std::net::{SocketAddr};
use std::io::ErrorKind;

#[derive(Debug, Copy, Clone)]
enum Quality {
    CRC_OK,
    CRC_FAIL,
    MISSED
}

struct LongFiPkt {
    oui: u32,
    device_id: u16,
    packet_id: u8,
    mac: u16,
    payload: Vec<u8>,
    num_fragments: u8,
    fragment_cnt: u8,
    quality: Vec<Quality>
}

impl core::fmt::Debug for LongFiPkt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {

        let mut quality = String::from("");

        for i in self.quality.iter() {
            match i {
                Quality::CRC_OK => quality.push('O'),
                Quality::CRC_FAIL=> quality.push('S'),
                Quality::MISSED => quality.push('X'),
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

enum ParserResponse {
    PKT(LongFiPkt),
    FRAGMENTED_PACKET_BEGIN(usize),
}

impl LongFiParser {
    fn new() -> LongFiParser {
        LongFiParser {
            fragmented_packets: array_of_none_256!(),
        }
    }

    fn timeout(&mut self, index: usize) -> Option<ParserResponse> {
        if let Some(mut pkt) = self.fragmented_packets[index].take() {

            while pkt.num_fragments > pkt.fragment_cnt {
                pkt.fragment_cnt += 1;
                pkt.quality.push(Quality::MISSED);
            }

            return Some(ParserResponse::PKT(pkt));

        }
        return None;
    }

    fn parse(&mut self, msg: &msg::Resp) -> Option<ParserResponse> {
        if let Some(message) = &msg.kind {
            if let msg::Resp_oneof_kind::rx_packet(lorapkt) = &message {

                // means single frament packet header
                if lorapkt.payload[0] == 0 {
                    let len_copy = lorapkt.payload.len() - PAYLOAD_BEGIN_SINGLE_FRAGMENT_PACKET;
                    let mut payload: Vec<u8> = vec![0; len_copy];
                    payload.copy_from_slice(
                        &lorapkt.payload[PAYLOAD_BEGIN_SINGLE_FRAGMENT_PACKET
                            ..PAYLOAD_BEGIN_SINGLE_FRAGMENT_PACKET + len_copy],
                    );

                    let mut quality = Vec::new();
                    if lorapkt.crc_check {
                        quality.push(Quality::CRC_OK);
                    }
                    else {
                        quality.push(Quality::CRC_FAIL);
                    }

                    return Some(ParserResponse::PKT(LongFiPkt {
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
                        quality,
                    }));
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

                    let mut quality = Vec::new();
                    if lorapkt.crc_check {
                        quality.push(Quality::CRC_OK);
                    }
                    else {
                        quality.push(Quality::CRC_FAIL);
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
                            mac: (lorapkt.payload[9] as u16)
                                | (lorapkt.payload[10] as u16) << 8,
                            payload,
                            quality,
                        }
                    });

                    return Some(ParserResponse::FRAGMENTED_PACKET_BEGIN(packet_id));

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

                        while fragment_num > pkt.fragment_cnt {
                            pkt.fragment_cnt += 1;
                            pkt.quality.push(Quality::MISSED);
                        }

                        if fragment_num == pkt.fragment_cnt {

                            if lorapkt.crc_check {
                                pkt.quality.push(Quality::CRC_OK);
                            }
                            else {
                                pkt.quality.push(Quality::CRC_FAIL);
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

use mio::{Events, Poll, PollOpt, Ready, Token};
use mio::udp::UdpSocket;
use mio_extras::timer::{Timer, Timeout};
use std::time::Duration;

struct Router;
impl Router {
    pub fn receive_msg() {}
}

const RECV_EVENT: Token = Token(0);

const PACKET_TIMEOUT: Token = Token(1);


pub fn longfi(print_level: u8, resp_port: u16) -> AppResult {
    let resp_addr = SocketAddr::from(([0, 0, 0, 0], resp_port));

    debug!("listening for responses on {}", resp_addr);
    let socket = UdpSocket::bind(&resp_addr)?;

    let mut read_buf = [0; 1024];
    let mut longfi = LongFiParser::new();

    // send beacon on start
    let poll = Poll::new().expect("Error initializing poll object");
    let mut timer: Timer<usize> = Timer::default();
    let mut timeouts: [Option<Timeout>;256] = array_of_none_256!();
    poll.register(&timer, PACKET_TIMEOUT, Ready::readable(), PollOpt::edge())
        .unwrap();

    poll.register(&socket, RECV_EVENT, Ready::readable(), PollOpt::edge())
        .unwrap();
    //timer.set_timeout(Duration::new(1, 0), "Beacon".to_string());
    let mut events = Events::with_capacity(128);
    loop {

        poll.poll(&mut events, None)
            .expect("Error receiving events from Epoll");

        for event in &events {
            let mut maybe_response = None;
            match event.token() {
                RECV_EVENT => {
                    if let Some(sz) = socket.recv_from(&mut read_buf)?{
                        match parse_from_bytes::<msg::Resp>(&read_buf[..sz.0]) {
                            Ok(rx_pkt) => {
                                maybe_response  = longfi.parse(&rx_pkt);   
                            }
                            Err(e) => error!("{:?}", e),
                        }
                    }
                    

                }
                PACKET_TIMEOUT => {
                    if let Some(index) = timer.poll() {
                        maybe_response = longfi.timeout(index);   
                    }
                }
                _ => (),
            }

            if let Some(response) = maybe_response {
                match response {
                    ParserResponse::PKT(pkt) => {
                        if let Some(timeout) = timeouts[pkt.packet_id as usize].take() {
                            timer.cancel_timeout(&timeout);
                        }
                        super::print_at_level(print_level, &pkt)
                    },
                    ParserResponse::FRAGMENTED_PACKET_BEGIN(index) => {
                        timeouts[index] = Some(timer.set_timeout(Duration::new(4, 0), index));
                    },
                }
            }
        }
       
    }
}

