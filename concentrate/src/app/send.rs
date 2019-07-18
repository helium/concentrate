use super::{msg_send, print_at_level};
use crate::error::{AppError, AppResult};
use messages as msg;
use protobuf::parse_from_bytes;
use std::{
    error::Error,
    io::ErrorKind,
    net::{SocketAddr, UdpSocket},
    time::Duration,
};

#[allow(clippy::too_many_arguments)]
pub fn send(
    print_level: u8,
    req_port: u16,
    resp_port: u16,
    implicit: bool,
    freq: u32,
    radio: u8,
    power: i8,
    spreading: u8,
    coderate: u8,
    bandwidth: u32,
    payload: Option<String>,
) -> AppResult {
    let tx_req = msg::RadioTxReq {
        freq,
        radio: match radio {
            0 => msg::Radio::R0,
            1 => msg::Radio::R1,
            e => return Err(AppError::Generic(format!("{} is not a valid radio", e))),
        },
        power: i32::from(power),
        bandwidth: match bandwidth {
            7800 => msg::Bandwidth::BW7_8kHz,
            15600 => msg::Bandwidth::BW15_6kHz,
            31200 => msg::Bandwidth::BW31_2kHz,
            62500 => msg::Bandwidth::BW62_5kHz,
            125_000 => msg::Bandwidth::BW125kHz,
            250_000 => msg::Bandwidth::BW250kHz,
            500_000 => msg::Bandwidth::BW500kHz,
            e => {
                return Err(AppError::Generic(format!("{} is not a valid bandwidth", e)));
            }
        },
        spreading: match spreading {
            7 => msg::Spreading::SF7,
            8 => msg::Spreading::SF8,
            9 => msg::Spreading::SF9,
            10 => msg::Spreading::SF10,
            11 => msg::Spreading::SF11,
            12 => msg::Spreading::SF12,
            e => {
                return Err(AppError::Generic(format!(
                    "{} is not a valid spreading factor",
                    e
                )));
            }
        },
        coderate: match coderate {
            5 => msg::Coderate::CR4_5,
            6 => msg::Coderate::CR4_6,
            7 => msg::Coderate::CR4_7,
            8 => msg::Coderate::CR4_8,
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
    debug!("requesting to transmit {:#?}", tx_req);

    let req_addr = SocketAddr::from(([127, 0, 0, 1], req_port));
    let socket = UdpSocket::bind(SocketAddr::from(([127, 0, 0, 1], resp_port)))?;
    socket.set_read_timeout(Some(Duration::from_millis(200)))?;
    msg_send(
        msg::RadioReq {
            id: 0xfe,
            kind: Some(msg::RadioReq_oneof_kind::tx(tx_req)),
            ..Default::default()
        },
        &socket,
        req_addr,
    )?;

    let mut read_buf = [0; 1024];
    match socket.recv(&mut read_buf) {
        Ok(sz) => match parse_from_bytes::<msg::RadioResp>(&read_buf[..sz]) {
            Ok(resp) => {
                print_at_level(print_level, &resp);
                Ok(())
            }
            Err(e) => {
                error!("{:?}", e);
                Err(AppError::Generic(e.description().into()))
            }
        },
        Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
            Err(AppError::Generic("timed out waiting for response".into()))
        }
        Err(e) => Err(e.into()),
    }
}
