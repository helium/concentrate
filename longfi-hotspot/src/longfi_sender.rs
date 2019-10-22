use super::LongFiResponse;
use lfc;
use messages as msg;
use std::collections::VecDeque;

pub struct LongFiSender {
    rng: rand::ThreadRng,
    req_id: Option<u32>,
    pending_fragments: Option<VecDeque<msg::RadioReq>>,
}

impl LongFiSender {
    pub fn new() -> LongFiSender {
        LongFiSender {
            rng: rand::thread_rng(),
            req_id: None,
            pending_fragments: None,
        }
    }

    pub fn tx_resp(&mut self) -> Option<LongFiResponse> {
        debug!("[LongFi] Radio has signalled that packet was sent");

        let mut clear_pending_fragments = false;

        let ret = match &mut self.pending_fragments {
            // if there is a vector, we should have more fragments
            Some(vec) => {
                debug!("[LongFi] Sending another fragment. {} remaining", vec.len());
                let maybe_fragment = vec.pop_front();

                if vec.is_empty() {
                    clear_pending_fragments = true;
                }

                match maybe_fragment {
                    Some(fragment) => {
                        // print the fragment payload nicely
                        if let Some(msg) = &fragment.kind {
                            let msg::RadioReq_oneof_kind::tx(tx) = msg;
                            debug!("[LongFi] Fragment: {:?}", tx.payload);
                        }

                        Some(LongFiResponse::RadioReq(fragment))
                    }
                    None => None,
                }
            }
            // if None, just completed a full packet
            None => match self.req_id.take() {
                Some(id) => {
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
                None => None,
            },
        };

        if clear_pending_fragments {
            self.pending_fragments = None;
        }
        ret
    }

    pub fn tx_uplink(&mut self, tx_uplink: &msg::LongFiTxUplinkPacket) -> Option<LongFiResponse> {
        if let Some(pkt) = lfc::serialize(&mut self.rng, tx_uplink) {
            Some(LongFiResponse::RadioReq(pkt))
        } else {
            None
        }
    }
}
