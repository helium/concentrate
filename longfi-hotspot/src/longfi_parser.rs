use super::{LongFiPkt, LongFiResponse};
use lfc;

pub struct LongFiParser {
    fragmented_packets: Box<[Option<LongFiPkt>]>,
}

impl LongFiParser {
    pub fn new() -> LongFiParser {
        LongFiParser {
            fragmented_packets: (0..256)
                .map(|_| None)
                .collect::<Vec<Option<LongFiPkt>>>()
                .into_boxed_slice(),
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
