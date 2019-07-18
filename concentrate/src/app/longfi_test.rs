use crate::error::AppResult;
use messages as msg;
use messages::LongFiRxPacket;
use mio::net::UdpSocket;
use mio::{Events, Poll, PollOpt, Ready, Token};
use mio_extras::timer::{Timeout, Timer};
use protobuf::{parse_from_bytes, Message};
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

use super::print_at_level;
extern crate rand;
use rand::Rng;

const PACKET_RECV_EVENT: Token = Token(0);
const PACKET_SEND_EVENT: Token = Token(1);

fn msg_send<T: Message>(msg: T, socket: &UdpSocket, addr: &SocketAddr) -> AppResult {
    let mut enc_buf = Vec::new();
    msg.write_to_vec(&mut enc_buf)
        .expect("error serializing response");
    socket.send_to(&enc_buf, addr)?;
    Ok(())
}

pub fn longfi_test(print_level: u8, ip: Option<IpAddr>, out_port: u16, in_port: u16) -> AppResult {
    let (socket, addr_out) = {
        let addr_in;
        let addr_out;

        if let Some(remote_ip) = ip {
            addr_in = SocketAddr::from(([0, 0, 0, 0], in_port));
            addr_out = SocketAddr::from((remote_ip, out_port));
        } else {
            addr_in = SocketAddr::from(([127, 0, 0, 1], in_port));
            addr_out = SocketAddr::from(([127, 0, 0, 1], out_port));
        }

        assert_ne!(addr_in, addr_out);
        println!("addr_in : {}", addr_in);
        println!("addr_out: {}", addr_out);
        (UdpSocket::bind(&addr_in)?, addr_out)
    };

    let mut read_buf = [0; 1024];

    let poll = Poll::new().expect("Error initializing poll object");

    poll.register(
        &socket,
        PACKET_RECV_EVENT,
        Ready::readable(),
        PollOpt::edge(),
    )
    .unwrap();

    let mut rng = rand::thread_rng();

    let payload: Vec<u8> = vec![rng.gen::<u8>(), rng.gen::<u8>()];

    let tx_req = msg::LongFiTxUplinkPacket {
        disable_encoding: true,
        disable_fragmentation: true,
        payload,
        ..Default::default()
    };
    //println!("requesting to transmit {:#?}", tx_req);
    msg_send(
        msg::Req {
            id: 0xfe,
            kind: Some(msg::Req_oneof_kind::longfi_tx_uplink(tx_req)),
            ..Default::default()
        },
        &socket,
        &addr_out,
    )?;

    let mut events = Events::with_capacity(128);
    loop {
        poll.poll(&mut events, None)
            .expect("Error receiving events from Epoll");

        for event in &events {
            // handle epoll events
            let maybe_response = match event.token() {
                PACKET_RECV_EVENT => {
                    println!("Received packet!");
                    // packet received from server
                    let sz = socket.recv(&mut read_buf)?;

                    // parse it into a raw packet
                    if let Ok(rx) = parse_from_bytes::<msg::Resp>(&read_buf[..sz]) {
                        println!("{:?}", rx)
                    }
                }
                _ => (),
            };
        }
    }
}
