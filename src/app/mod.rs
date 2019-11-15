use crate::error::AppResult;
use protobuf::Message;
use std::{
    fmt,
    net::{SocketAddr, UdpSocket},
};

mod bist;
mod connect;
mod listen;
mod longfi;
mod longfi_test;
mod send;
mod serve;

pub use self::bist::*;
pub use self::connect::*;
pub use self::listen::*;
pub use self::longfi::*;
pub use self::longfi_test::*;
pub use self::send::*;
pub use self::serve::*;

fn print_at_level<T: fmt::Debug>(print_level: u8, pkt: &T) {
    match print_level {
        0 => (),
        1 => println!("{:?}\n", pkt),
        _ => println!("{:#?}\n", pkt),
    }
}

fn msg_send<T: Message>(msg: T, socket: &UdpSocket, addr: SocketAddr) -> AppResult {
    let mut enc_buf = Vec::new();
    msg.write_to_vec(&mut enc_buf)
        .expect("error serializing response");
    socket.send_to(&enc_buf, addr)?;
    Ok(())
}
