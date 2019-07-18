use crate::error::AppResult;
use protobuf::Message;
use std::fmt;
use std::net::{SocketAddr, UdpSocket};

mod bist;
mod listen;
mod longfi;
mod longfi_test;
mod send;
mod serve;

pub use self::bist::*;
pub use self::listen::*;
pub use self::longfi::*;
pub use self::longfi_test::*;
pub use self::send::*;
pub use self::serve::*;

fn print_at_level<T: fmt::Debug>(print_level: u8, pkt: &T) {
    if print_level > 1 {
        println!("{:#?}\n", pkt);
    } else if print_level == 1 {
        println!("{:?}\n", pkt);
    }
}

fn msg_send<T: Message>(msg: T, socket: &UdpSocket, addr: SocketAddr) -> AppResult {
    let mut enc_buf = Vec::new();
    msg.write_to_vec(&mut enc_buf)
        .expect("error serializing response");
    socket.send_to(&enc_buf, addr)?;
    Ok(())
}
