mod gateway;
pub use gateway::*;
use loragw;

impl From<loragw::Spreading> for Spreading {
    fn from(other: loragw::Spreading) -> Spreading {
        match other {
            loragw::Spreading::SF7 => Spreading::SF7,
            loragw::Spreading::SF8 => Spreading::SF8,
            loragw::Spreading::SF9 => Spreading::SF9,
            loragw::Spreading::SF10 => Spreading::SF10,
            loragw::Spreading::SF11 => Spreading::SF11,
            loragw::Spreading::SF12 => Spreading::SF12,
            _ => Spreading::UNDEFINED,
        }
    }
}

impl From<loragw::Bandwidth> for Bandwidth {
    fn from(other: loragw::Bandwidth) -> Bandwidth {
        match other {
            loragw::Bandwidth::BW7_8kHz => Bandwidth::BW7_8kHz,
            loragw::Bandwidth::BW15_6kHz => Bandwidth::BW15_6kHz,
            loragw::Bandwidth::BW31_2kHz => Bandwidth::BW31_2kHz,
            loragw::Bandwidth::BW62_5kHz => Bandwidth::BW62_5kHz,
            loragw::Bandwidth::BW125kHz => Bandwidth::BW125kHz,
            loragw::Bandwidth::BW250kHz => Bandwidth::BW250kHz,
            loragw::Bandwidth::BW500kHz => Bandwidth::BW500kHz,
            _ => Bandwidth::UNDEFINED,
        }
    }
}

impl From<loragw::Coderate> for Coderate {
    fn from(other: loragw::Coderate) -> Coderate {
        match other {
            loragw::Coderate::Cr4_5 => Coderate::CR4_5,
            loragw::Coderate::Cr4_6 => Coderate::CR4_6,
            loragw::Coderate::Cr4_7 => Coderate::CR4_7,
            loragw::Coderate::Cr4_8 => Coderate::CR4_8,
            _ => Coderate::UNDEFINED,
        }
    }
}

impl From<loragw::RxPacketLoRa> for RxPacket {
    fn from(other: loragw::RxPacketLoRa) -> RxPacket {
        RxPacket {
            freq: other.freq,
            if_chain: other.if_chain.into(),
            crc_check: match other.crc_check {
                loragw::CRCCheck::Fail => false,
                _ => true,
            },
            timestamp: other.timestamp.as_micros() as u64,
            radio: other.radio as u32,
            bandwidth: other.bandwidth.into(),
            spreading: other.spreading.into(),
            coderate: other.coderate.into(),
            rssi: other.rssi,
            snr: other.snr,
            payload: other.payload,
            ..Default::default()
        }
    }
}

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
