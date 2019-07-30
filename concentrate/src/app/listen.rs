use crate::{cmdline, error::AppResult};
use messages as msg;
use protobuf::parse_from_bytes;
use std::net::{SocketAddr, UdpSocket};

pub fn listen(args: cmdline::Listen) -> AppResult {
    let listen_addr = SocketAddr::from(([0, 0, 0, 0], args.listen_port));

    debug!("listening for responses on {}", listen_addr);
    let socket = UdpSocket::bind(listen_addr)?;

    let mut read_buf = [0; 1024];

    loop {
        let (sz, src) = socket.recv_from(&mut read_buf)?;
        debug!("read {} bytes from {}", sz, src);
        match parse_from_bytes::<msg::RadioResp>(&read_buf[..sz]) {
            Ok(rx_pkt) => super::print_at_level(args.print_level, &rx_pkt),
            Err(e) => error!("{:?}", e),
        }
    }
}
