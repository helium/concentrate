use crate::error::AppResult;
use protobuf::parse_from_bytes;
use std::net::{SocketAddr, UdpSocket};

pub fn listen(print_level: u8, publish_port: u16) -> AppResult {
    let publish_addr = SocketAddr::from(([127, 0, 0, 1], publish_port));
    debug!("listening for published packets on {}", publish_addr);
    let socket = UdpSocket::bind(publish_addr)?;

    let mut udp_read_buf = [0; 1024];

    loop {
        let (sz, src) = socket.recv_from(&mut udp_read_buf)?;
        debug!("read {} bytes from {}", sz, src);
        match parse_from_bytes::<messages::RxPacket>(&udp_read_buf[..sz]) {
            Ok(rx_pkt) => super::print_at_level(print_level, &rx_pkt),
            Err(e) => error!("{:?}", e),
        }
    }
}
