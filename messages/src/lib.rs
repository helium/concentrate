extern crate loragw;
extern crate protobuf;

mod gateway;
pub use gateway::*;

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

impl From<Spreading> for loragw::Spreading {
    fn from(other: Spreading) -> loragw::Spreading {
        match other {
            Spreading::SF7 => loragw::Spreading::SF7,
            Spreading::SF8 => loragw::Spreading::SF8,
            Spreading::SF9 => loragw::Spreading::SF9,
            Spreading::SF10 => loragw::Spreading::SF10,
            Spreading::SF11 => loragw::Spreading::SF11,
            Spreading::SF12 => loragw::Spreading::SF12,
            _ => loragw::Spreading::Undefined,
        }
    }
}

impl From<loragw::Bandwidth> for Bandwidth {
    fn from(other: loragw::Bandwidth) -> Bandwidth {
        match other {
            loragw::Bandwidth::BW125kHz => Bandwidth::BW125kHz,
            loragw::Bandwidth::BW250kHz => Bandwidth::BW250kHz,
            loragw::Bandwidth::BW500kHz => Bandwidth::BW500kHz,
            _ => Bandwidth::UNDEFINED,
        }
    }
}

impl From<Bandwidth> for loragw::Bandwidth {
    fn from(other: Bandwidth) -> loragw::Bandwidth {
        match other {
            Bandwidth::BW125kHz => loragw::Bandwidth::BW125kHz,
            Bandwidth::BW250kHz => loragw::Bandwidth::BW250kHz,
            Bandwidth::BW500kHz => loragw::Bandwidth::BW500kHz,
            _ => loragw::Bandwidth::Undefined,
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

impl From<Coderate> for loragw::Coderate {
    fn from(other: Coderate) -> loragw::Coderate {
        match other {
            Coderate::CR4_5 => loragw::Coderate::Cr4_5,
            Coderate::CR4_6 => loragw::Coderate::Cr4_6,
            Coderate::CR4_7 => loragw::Coderate::Cr4_7,
            Coderate::CR4_8 => loragw::Coderate::Cr4_8,
            _ => loragw::Coderate::Undefined,
        }
    }
}

impl From<loragw::Radio> for Radio {
    fn from(other: loragw::Radio) -> Radio {
        match other {
            loragw::Radio::R0 => Radio::R0,
            loragw::Radio::R1 => Radio::R1,
        }
    }
}

impl From<Radio> for loragw::Radio {
    fn from(other: Radio) -> loragw::Radio {
        match other {
            Radio::R0 => loragw::Radio::R0,
            Radio::R1 => loragw::Radio::R1,
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
            radio: other.radio.into(),
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

impl From<TxReq> for loragw::TxPacketLoRa {
    fn from(other: TxReq) -> loragw::TxPacketLoRa {
        loragw::TxPacketLoRa {
            freq: other.freq,
            mode: loragw::TxMode::Immediate,
            radio: other.radio.into(),
            power: other.power as i8,
            bandwidth: other.bandwidth.into(),
            spreading: other.spreading.into(),
            coderate: other.coderate.into(),
            invert_polarity: other.invert_polarity,
            preamble: None,
            omit_crc: other.omit_crc,
            implicit_header: other.implicit_header,
            payload: other.payload,
        }
    }
}
