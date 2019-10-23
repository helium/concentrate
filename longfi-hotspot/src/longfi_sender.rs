use super::LongFiResponse;
use lfc;
use messages as msg;
use std::collections::VecDeque;

pub struct LongFiSender {
    rng: rand::ThreadRng,
    req_id: Option<u32>,
    pending_fragments: VecDeque<msg::RadioReq>,
}

impl LongFiSender {
    pub fn new() -> LongFiSender {
        LongFiSender {
            rng: rand::thread_rng(),
            req_id: None,
            pending_fragments: VecDeque::new(),
        }
    }

    pub fn tx_resp(&mut self) -> Option<LongFiResponse> {
        debug!("[LongFi] Radio has signalled that packet was sent");

        let vec = &mut self.pending_fragments;

        match vec.pop_front() {
            Some(fragment) => {
                debug!("[LongFi] Sending another fragment. {} remaining", vec.len());
                // print the fragment payload nicely
                if let Some(msg) = &fragment.kind {
                    let msg::RadioReq_oneof_kind::tx(tx) = msg;
                    debug!("[LongFi] Fragment: {:?}", tx.payload);
                }

                Some(LongFiResponse::RadioReq(fragment))
            }
            // no fragments means we are all done
            None => {
                let id = match self.req_id.take() {
                    Some(id) => id,
                    None => 0,
                };
                debug!("[LongFi] All packet fragments transmitted");
                Some(LongFiResponse::ClientResp(msg::LongFiResp {
                    id,
                    kind: Some(msg::LongFiResp_oneof_kind::tx_status(msg::LongFiTxStatus {
                        success: true,
                        ..Default::default()
                    })),
                    ..Default::default()
                }))
            }
        }
    }

    pub fn transmit(&mut self, tx_uplink: &msg::LongFiTxPacket) -> Option<LongFiResponse> {
        if let Some(pkt) = lfc::serialize(&mut self.rng, tx_uplink) {
            Some(LongFiResponse::RadioReq(pkt))
        } else {
            None
        }
    }
}
