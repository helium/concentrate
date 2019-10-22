use super::LongFiResponse;
use lfc;

pub struct LongFiParser {
}

impl LongFiParser {
    pub fn new() -> LongFiParser {
        LongFiParser {
        }
    }

    pub fn parse(&mut self, pkt: &messages::RadioRxPacket) -> Option<LongFiResponse> {
        if let Some(req) = lfc::parse(pkt) {
            Some(LongFiResponse::PktRx(req))
        } else {
            None
        }
    }
}
