extern crate messages;
extern crate mio;
extern crate protobuf;
extern crate rand;
#[macro_use]
extern crate log;

use messages as msg;

#[cfg(test)]
mod tests;

mod longfi_parser;
mod longfi_sender;

use crate::longfi_parser::LongFiParser;
use crate::longfi_sender::LongFiSender;
use lfc::LongFiPkt;

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
}

impl Default for LongFi {
    fn default() -> LongFi {
        LongFi::new()
    }
}
