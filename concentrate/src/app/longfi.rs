use crate::error::AppResult;
use longfi_hotspot::{LongFiParser, ParserResponse};
use messages as msg;
use mio::net::UdpSocket;
use mio::{Events, Poll, PollOpt, Ready, Token};
use mio_extras::timer::{Timeout, Timer};
use protobuf::{parse_from_bytes, Message};
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use messages::LongFiRxPacket;

use super::print_at_level;

const RECV_EVENT: Token = Token(0);
const PACKET_TIMEOUT: Token = Token(1);

fn msg_send<T: Message>(msg: T, socket: &UdpSocket, addr: &SocketAddr) -> AppResult {
    let mut enc_buf = Vec::new();
    msg.write_to_vec(&mut enc_buf)
        .expect("error serializing response");
    socket.send_to(&enc_buf, addr)?;
    Ok(())
}

pub fn longfi(print_level: u8, out_port: u16, in_port: u16, ip: Option<IpAddr>, longfi_out_port: u16, longfi_in_port:u16 ) -> AppResult {
    let (socket, addr_out, longfi_socket, longfi_addr_out) = {
        let addr_in;
        let addr_out;

        if let Some(remote_ip) = ip {
            addr_in = SocketAddr::from(([0,0,0,0], in_port));
            addr_out = SocketAddr::from((remote_ip, out_port));
        } else {
            addr_in = SocketAddr::from(([127, 0, 0, 1], in_port));
            addr_out = SocketAddr::from(([127, 0, 0, 1], out_port));
        }

        let longfi_addr_in = SocketAddr::from(([127, 0, 0, 1], longfi_in_port));
        let longfi_addr_out = SocketAddr::from(([127, 0, 0, 1], longfi_out_port));

        assert_ne!(addr_in, addr_out);
        println!("addr_in : {}", addr_in);
        println!("addr_out: {}", addr_out);
        (
            UdpSocket::bind(&addr_in)?,
            addr_out,
            UdpSocket::bind(&longfi_addr_in)?,
            longfi_addr_out,
        )
    };

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
            // handle epoll events
            let maybe_response = match event.token() {
                RECV_EVENT => {
                    let sz = socket.recv(&mut read_buf)?;
                    match parse_from_bytes::<msg::Resp>(&read_buf[..sz]) {
                        Ok(rx) => longfi.parse(&rx),
                        Err(e) => {
                            error!("{:?}", e);
                            None
                        }
                    }
                }
                PACKET_TIMEOUT => {
                    if let Some(index) = timer.poll() {
                        longfi.timeout(index)
                    } else {
                        None
                    }
                }
                _ => None,
            };

            // if there was a response, deal with it
            if let Some(response) = maybe_response {
                match response {
                    ParserResponse::Pkt(pkt) => {
                        if let Some(timeout) = timeouts[pkt.packet_id as usize].take() {
                            timer.cancel_timeout(&timeout);
                        }
                        println!("{:?}", pkt);

                        let resp = msg::Resp {
                            id: 0,
                            kind: Some(msg::Resp_oneof_kind::longfi_rx_packet(pkt.into())),
                            ..Default::default()
                        };

                        msg_send(resp, &longfi_socket, &longfi_addr_out)?;
                    }
                    ParserResponse::FragmentedPacketBegin(index) => {
                        timeouts[index] = Some(timer.set_timeout(Duration::new(4, 0), index));
                    }
                }
            }
        }
    }
}
