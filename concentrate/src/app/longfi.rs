use crate::error::AppResult;
use messages as msg;
use protobuf::parse_from_bytes;
use std::net::{SocketAddr, UdpSocket};

struct LongFiPkt {
    freq: u32,
    oui: u32,
    device_id: u16,
    packet_id: u8,
    mac: u16,
    payload: Vec<u8>
}

impl core::fmt::Debug for LongFiPkt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        

        write!(
            f,
            "LongFiPkt {{ freq: {}, oui: 0x{:x}, device_id 0x{:x}, packet_id: 0x{:x}, mac: 0x{:x},
            payload: {:?} }}",
            self.freq,
            self.oui,
            self.device_id,
            self.packet_id,
            self.mac,
            self.payload
        )
    }
}

use byteorder::{ByteOrder, LittleEndian};

const PAYLOAD_BEGIN: usize = 9;

impl LongFiPkt {

    pub fn new(vec: Vec<u8>) -> LongFiPkt {
        LongFiPkt {
            freq: 0,
            oui: 0,
            device_id: 0,
            packet_id: 0,
            mac: 0,
            payload: vec
        }
    }

    pub fn from_msg_resp(msg: &msg::Resp) -> Option<LongFiPkt> {
        if let Some(message) = &msg.kind {
            if let msg::Resp_oneof_kind::rx_packet(lorapkt) = &message {
                    
                    if lorapkt.crc_check {
                        let len_copy = lorapkt.payload.len() - PAYLOAD_BEGIN;
                        let mut payload: Vec<u8> = vec![0; len_copy];
                        payload.copy_from_slice(&lorapkt.payload[PAYLOAD_BEGIN..PAYLOAD_BEGIN+len_copy]);

                        return Some(LongFiPkt {
                            freq: lorapkt.freq,
                            oui: (lorapkt.payload[0] as u32) |
                                 (lorapkt.payload[1] as u32) << 8 | 
                                 (lorapkt.payload[2] as u32) << 16 | 
                                 (lorapkt.payload[3] as u32) << 24,
                            device_id: (lorapkt.payload[4] as u16) | (lorapkt.payload[5] as u16) << 8,
                            packet_id: lorapkt.payload[6],
                            mac: (lorapkt.payload[7] as u16) | (lorapkt.payload[8] as u16) << 8,

                            payload,
                        });    
                    }           
            }
        }
        
        None
    }
}

pub fn longfi(print_level: u8, resp_port: u16) -> AppResult {
    let resp_addr = SocketAddr::from(([0, 0, 0, 0], resp_port));

    debug!("listening for responses on {}", resp_addr);
    let socket = UdpSocket::bind(resp_addr)?;

    let mut read_buf = [0; 1024];

    loop {
        let (sz, src) = socket.recv_from(&mut read_buf)?;
        match parse_from_bytes::<msg::Resp>(&read_buf[..sz]) {
            Ok(rx_pkt) => {
                let longfi_pkt = LongFiPkt::from_msg_resp(&rx_pkt);
                if let Some(pkt) = longfi_pkt{
                    super::print_at_level(print_level, &pkt);
                }
            },
            Err(e) => error!("{:?}", e),
        }
    }
}
