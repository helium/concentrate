use crate::{cmdline, error::AppResult};
use messages as msg;
use protobuf::parse_from_bytes;
use std::net::UdpSocket;

pub fn listen(args: cmdline::Listen) -> AppResult {
    log::debug!("listening for responses on {}", args.listen_addr_in);
    let socket = UdpSocket::bind(args.listen_addr_in)?;

    let mut read_buf = [0; 1024];

    loop {
        let (sz, src) = socket.recv_from(&mut read_buf)?;
        log::debug!("read {} bytes from {}", sz, src);
        match parse_from_bytes::<msg::RadioResp>(&read_buf[..sz]) {
            Ok(rx_pkt) => super::print_at_level(args.print_level, &rx_pkt),
            Err(e) => log::error!("{:?}", e),
        }
    }
}
