use crate::error::AppResult;
use messages as msg;
use mio::net::UdpSocket;
use mio::{Events, Poll, PollOpt, Ready, Token};
use protobuf::{parse_from_bytes, Message};
use std::net::{IpAddr, SocketAddr};

extern crate rand;
use rand::Rng;

const PACKET_RECV_EVENT: Token = Token(0);

fn msg_send<T: Message>(msg: T, socket: &UdpSocket, addr: &SocketAddr) -> AppResult {
    let mut enc_buf = Vec::new();
    msg.write_to_vec(&mut enc_buf)
        .expect("error serializing response");
    socket.send_to(&enc_buf, addr)?;
    Ok(())
}

pub fn longfi_test(_print_level: u8, ip: Option<IpAddr>, out_port: u16, in_port: u16) -> AppResult {
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
        debug!("addr_in : {}", addr_in);
        debug!("addr_out: {}", addr_out);
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
    let payload: Vec<u8> = (0..90).map(|_| rng.gen::<u8>()).collect();

    println!("sending packet: {:?}", payload);
    let tx_req = msg::LongFiTxUplinkPacket {
        disable_encoding: false,
        disable_fragmentation: false,
        oui: 0xBEEFFEED,
        device_id: 0xABCD,
        spreading: msg::LongFiSpreading::SF10,
        payload,
        ..Default::default()
    };

    msg_send(
        msg::LongFiReq {
            id: 0xfe,
            kind: Some(msg::LongFiReq_oneof_kind::tx_uplink(tx_req)),
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
            let _maybe_response = match event.token() {
                PACKET_RECV_EVENT => {
                    // packet received from server
                    let sz = socket.recv(&mut read_buf)?;

                    // parse it into a raw packet
                    if let Ok(rx) = parse_from_bytes::<msg::LongFiResp>(&read_buf[..sz]) {
                        match &rx.kind {
                            Some(request) => match request {
                                msg::LongFiResp_oneof_kind::rx(pkt) => {
                                    println!("Received packet! Length = {}", pkt.payload.len());

                                    for byte in &pkt.payload {
                                        print!("{:} ", *byte as u8);
                                    }
                                    println!("");
                                }
                                _ => (),
                            },
                            None => (),
                        }
                    } else {
                        println!("Failed to parse LongFiResp!");
                    }
                }
                _ => (),
            };
        }
    }
}
