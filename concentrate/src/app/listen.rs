use crate::error::AppResult;
use messages as msg;
use protobuf::parse_from_bytes;
use std::net::{SocketAddr, UdpSocket};

pub fn listen(print_level: u8, resp_port: u16) -> AppResult {
    let resp_addr = SocketAddr::from(([0, 0, 0, 0], resp_port));

    debug!("listening for responses on {}", resp_addr);
    let socket = UdpSocket::bind(resp_addr)?;

    let mut read_buf = [0; 1024];

    loop {
        let (sz, src) = socket.recv_from(&mut read_buf)?;
        debug!("read {} bytes from {}", sz, src);
        match parse_from_bytes::<msg::Resp>(&read_buf[..sz]) {
            Ok(rx_pkt) => super::print_at_level(print_level, &rx_pkt),
            Err(e) => error!("{:?}", e),
        }
    }
}
