extern crate messages;
extern crate mio;
extern crate protobuf;
extern crate rand;
#[macro_use]
extern crate log;
use std::convert::From;

use messages as msg;

#[cfg(test)]
mod tests;

mod longfi_parser;
mod longfi_sender;

use longfi_parser::LongFiParser;
use longfi_sender::LongFiSender;

use msg::LongFiSpreading as Spreading;

#[derive(Debug)]
pub enum LongFiResponse {
    PktRx(LongFiPkt),
    RadioReq(msg::RadioReq),
    ClientResp(msg::LongFiResp),
}

pub struct LongFi {
    parser: LongFiParser,
    sender: LongFiSender,
}

impl LongFi {
    pub fn new() -> LongFi {
        LongFi {
            parser: LongFiParser::new(),
            sender: LongFiSender::new(),
        }
    }

    pub fn handle_response(&mut self, resp: &msg::RadioResp) -> Option<LongFiResponse> {
        match &resp.kind {
            Some(resp) => match resp {
                msg::RadioResp_oneof_kind::tx(_) => self.sender.tx_resp(),
                msg::RadioResp_oneof_kind::rx_packet(rx) => self.parser.parse(rx),
                msg::RadioResp_oneof_kind::parse_err(_) => None,
            },
            None => None,
        }
    }

    pub fn handle_request(&mut self, req: &msg::LongFiReq) -> Option<LongFiResponse> {
        match &req.kind {
            Some(request) => match request {
                msg::LongFiReq_oneof_kind::tx_uplink(tx_uplink) => self.sender.tx_uplink(tx_uplink),
            },
            None => None,
        }
    }

    pub fn parser_timeout(&mut self, index: usize) -> Option<LongFiResponse> {
        self.parser.timeout(index)
    }
}

impl Default for LongFi {
    fn default() -> LongFi {
        LongFi::new()
    }
}

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
    timestamp: u64,
    snr: f32,
    rssi: f32,
    spreading: Spreading,
    quality: Vec<Quality>,
    crc_fails: usize,
}

impl LongFiPkt {
    pub fn quality_string(&self) -> String {
        let mut quality = String::new();

        for i in &self.quality {
            match i {
                Quality::CrcOk => quality.push('O'),
                Quality::CrcFail => quality.push('S'),
                Quality::Missed => quality.push('X'),
            }
        }

        quality
    }
}

use messages::LongFiRxPacket;

impl From<LongFiPkt> for LongFiRxPacket {
    fn from(other: LongFiPkt) -> LongFiRxPacket {
        let mut crc_check = true;

        for i in other.quality {
            if i != Quality::CrcOk {
                crc_check = false;
                break;
            }
        }

        LongFiRxPacket {
            crc_check,
            timestamp: other.timestamp,
            rssi: other.rssi,
            snr: other.snr,
            oui: other.oui as u32,
            device_id: u32::from(other.device_id),
            mac: u32::from(other.mac),
            payload: other.payload,
            spreading: other.spreading,
            // special fields
            unknown_fields: Default::default(),
            cached_size: Default::default(),
        }
    }
}

impl core::fmt::Debug for LongFiPkt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let quality = self.quality_string();

        write!(
            f,
            "LongFiPkt {{ oui: 0x{:x}, device_id 0x{:x}, packet_id: 0x{:x}, mac: 0x{:x}, quality: {}, payload: {:?} }}",
            self.oui, self.device_id, self.packet_id, self.mac, quality, self.payload
        )
    }
}

impl PartialEq for LongFiPkt {
    fn eq(&self, other: &Self) -> bool {
        self.oui == other.oui && self.device_id == other.device_id && self.payload == other.payload
    }
}
