use crate::error::{AppError, AppResult};
use protobuf::Message;
use std::net::{SocketAddr, UdpSocket};

#[allow(clippy::too_many_arguments)]
pub fn send(
    implicit: bool,
    listen_port: u16,
    chan: Option<u32>,
    freq: Option<f64>,
    radio: u8,
    power: i8,
    spreading: u8,
    coderate: u8,
    bandwidth: u32,
    payload: Option<String>,
) -> AppResult {
    let chan_or_freq = match (chan, freq) {
        (Some(chan), None) => messages::TxPacket_oneof_freq_or_chan::chan(chan),
        (None, Some(freq)) => messages::TxPacket_oneof_freq_or_chan::freq(freq as u32),
        _ => panic!("argument parsing failed to notice the lack of a required option"),
    };

    let tx_pkt = messages::TxPacket {
        freq_or_chan: Some(chan_or_freq),
        radio: match radio {
            0 => messages::Radio::R0,
            1 => messages::Radio::R1,
            e => return Err(AppError::Generic(format!("{} is not a valid radio", e))),
        },
        power: i32::from(power),
        bandwidth: match bandwidth {
            7800 => messages::Bandwidth::BW7_8kHz,
            15600 => messages::Bandwidth::BW15_6kHz,
            31200 => messages::Bandwidth::BW31_2kHz,
            62500 => messages::Bandwidth::BW62_5kHz,
            125_000 => messages::Bandwidth::BW125kHz,
            250_000 => messages::Bandwidth::BW250kHz,
            500_000 => messages::Bandwidth::BW500kHz,
            e => {
                return Err(AppError::Generic(format!("{} is not a valid bandwidth", e)));
            }
        },
        spreading: match spreading {
            7 => messages::Spreading::SF7,
            8 => messages::Spreading::SF8,
            9 => messages::Spreading::SF9,
            10 => messages::Spreading::SF10,
            11 => messages::Spreading::SF11,
            12 => messages::Spreading::SF12,
            e => {
                return Err(AppError::Generic(format!(
                    "{} is not a valid spreading factor",
                    e
                )));
            }
        },
        coderate: match coderate {
            5 => messages::Coderate::CR4_5,
            6 => messages::Coderate::CR4_6,
            7 => messages::Coderate::CR4_7,
            8 => messages::Coderate::CR4_8,
            e => {
                return Err(AppError::Generic(format!(
                    "4/{} is not a valid coderate",
                    e
                )));
            }
        },
        invert_polarity: false,
        omit_crc: false,
        implicit_header: implicit,
        payload: payload.unwrap_or_default().into_bytes(),
        ..Default::default()
    };
    debug!("requesting to transmit {:#?}", tx_pkt);
    let socket = UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], 0)))?;
    let listen_addr = SocketAddr::from(([127, 0, 0, 1], listen_port));
    let mut tx_buf = Vec::new();
    tx_pkt
        .write_to_vec(&mut tx_buf)
        .expect("error serializing TxPacket");
    debug!("encoded TxPacket into {} bytes", tx_buf.len());
    socket.send_to(&tx_buf, listen_addr)?;
    Ok(())
}
