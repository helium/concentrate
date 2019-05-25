mod gateway;
pub use gateway::*;

#[cfg(test)]
mod tests {
    #[test]
    fn test_proto_round_trip() {
        use crate::gateway::*;
        use protobuf::{parse_from_bytes, Message};
        let mut buf = Vec::new();
        let mut pkt0 = RxPacket::new();
        pkt0.dummy = 42;
        pkt0.write_to_vec(&mut buf).unwrap();
        let pkt1: RxPacket = parse_from_bytes(&buf).unwrap();
        assert_eq!(pkt0.dummy, pkt1.dummy);
    }
}
