#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    // ^ Clippy triggers false-positive unused imports in this module,
    // hence `allow` above.
    use longfi_sender::{PacketHeader, PacketHeaderMultipleFragments};
    use messages as msg;
    use msg::LongFiSpreading as Spreading;
    use LongFi;
    use LongFiResponse;

    #[test]
    fn test_header_serialization() {
        let oui = 0x1234_5678;
        let device_id = 0x9ABC;
        let mac = 0xBEEF;
        let packet_id = 0xBB;

        let expected_array = [
            packet_id,
            oui as u8,
            (oui >> 8) as u8,
            (oui >> 16) as u8,
            (oui >> 24) as u8,
            device_id as u8,
            (device_id >> 8) as u8,
            mac as u8,
            (mac >> 8) as u8,
        ];

        let header_struct = PacketHeader {
            packet_id,
            oui,
            device_id,
            mac,
        };

        let serialized_array: [u8; 9] = header_struct.into();

        assert_eq!(expected_array, serialized_array);
    }

    #[test]
    fn test_multifragment_header_serialization() {
        let oui = 0x1234_5678;
        let device_id = 0x9ABC;
        let mac = 0xBEEF;
        let packet_id = 0xBB;
        let num_fragments = 0xAF;
        let expected_array = [
            packet_id,
            0,
            num_fragments,
            oui as u8,
            (oui >> 8) as u8,
            (oui >> 16) as u8,
            (oui >> 24) as u8,
            device_id as u8,
            (device_id >> 8) as u8,
            mac as u8,
            (mac >> 8) as u8,
        ];

        let header_struct = PacketHeaderMultipleFragments {
            packet_id,
            fragment_num: 0,
            num_fragments,
            oui,
            device_id,
            mac,
        };

        let serialized_array: [u8; 11] = header_struct.into();

        assert_eq!(expected_array, serialized_array);
    }

    #[test]
    fn test_single_fragment_transmit() {
        let mut longfi = LongFi::new();

        let payload = vec![1, 2, 3];

        let uplink_req = msg::LongFiTxUplinkPacket {
            disable_encoding: false,
            disable_fragmentation: false,
            oui: 0x1234_5678,
            device_id: 0x9ABC,
            spreading: Spreading::SF7,
            payload,
            ..Default::default()
        };

        let req = msg::LongFiReq {
            id: 0x99,
            kind: Some(msg::LongFiReq_oneof_kind::tx_uplink(uplink_req)),
            ..Default::default()
        };

        let longfi_response = longfi.handle_request(&req);

        match longfi_response {
            Some(response) => {
                match response {
                    LongFiResponse::RadioReq(msg) => {
                        match &msg.kind {
                            Some(req) => {
                                match req {
                                    msg::RadioReq_oneof_kind::tx(tx) => {
                                        for byte in &tx.payload {
                                            print!("{:x} ", byte);
                                        }
                                        println!();
                                    }
                                    _ => panic!("Wrong radio request!"),
                                };
                            }
                            None => panic!("Invalid protobuf message"),
                        };
                    }
                    _ => panic!("Wrong LongFi response given!"),
                };
            }
            None => panic!("No LongFi response given!"),
        }
    }
}
