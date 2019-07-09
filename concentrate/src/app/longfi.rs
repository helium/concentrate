use crate::error::AppResult;
use messages as msg;
use protobuf::parse_from_bytes;
use std::net::SocketAddr;

use longfi_hotspot::{LongFiParser, ParserResponse};
use mio::net::UdpSocket;
use mio::{Events, Poll, PollOpt, Ready, Token};
use mio_extras::timer::{Timeout, Timer};
use std::time::Duration;

const RECV_EVENT: Token = Token(0);
const PACKET_TIMEOUT: Token = Token(1);

pub fn longfi(print_level: u8, resp_port: u16) -> AppResult {
    let resp_addr = SocketAddr::from(([0, 0, 0, 0], resp_port));

    debug!("listening for responses on {}", resp_addr);
    let socket = UdpSocket::bind(&resp_addr)?;

    let mut read_buf = [0; 1024];
    let mut longfi = LongFiParser::new();

    let poll = Poll::new().expect("Error initializing poll object");
    let mut timer: Timer<usize> = Timer::default();
    let mut timeouts: [Option<Timeout>; 256] = array_of_none_256!();
    poll.register(&timer, PACKET_TIMEOUT, Ready::readable(), PollOpt::edge())
        .unwrap();

    poll.register(&socket, RECV_EVENT, Ready::readable(), PollOpt::edge())
        .unwrap();

    let mut events = Events::with_capacity(128);
    loop {
        poll.poll(&mut events, None)
            .expect("Error receiving events from Epoll");

        for event in &events {
            let mut maybe_response = None;
            match event.token() {
                RECV_EVENT => {
                    let sz = socket.recv(&mut read_buf)?;
                    match parse_from_bytes::<msg::Resp>(&read_buf[..sz]) {
                        Ok(rx_pkt) => {
                            maybe_response = longfi.parse(&rx_pkt);
                        }
                        Err(e) => error!("{:?}", e),
                    }
                }
                PACKET_TIMEOUT => {
                    if let Some(index) = timer.poll() {
                        maybe_response = longfi.timeout(index);
                    }
                }
                _ => (),
            }

            if let Some(response) = maybe_response {
                match response {
                    ParserResponse::PKT(pkt) => {
                        if let Some(timeout) = timeouts[pkt.packet_id as usize].take() {
                            timer.cancel_timeout(&timeout);
                        }
                        super::print_at_level(print_level, &pkt)
                    }
                    ParserResponse::FRAGMENTED_PACKET_BEGIN(index) => {
                        timeouts[index] = Some(timer.set_timeout(Duration::new(4, 0), index));
                    }
                }
            }
        }
    }
}
