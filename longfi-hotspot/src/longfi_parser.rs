use super::LongFiResponse;
use lfc;

pub struct LongFiParser {}

impl LongFiParser {
    pub fn new() -> LongFiParser {
        LongFiParser {}
    }

    pub fn parse(&mut self, pkt: &messages::RadioRxPacket) -> Option<LongFiResponse> {
        println!("Received raw LoRa packet");
        if let Some(req) = lfc::parse(pkt) {
            println!("Parsed packet {:?}", req);
            Some(LongFiResponse::PktRx(req))
        } else {
            None
        }
    }
}
