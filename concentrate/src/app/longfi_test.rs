use crate::{cmdline, error::AppResult};
use messages as msg;
use mio::net::UdpSocket;
use mio::{Events, Poll, PollOpt, Ready, Token};
use protobuf::{parse_from_bytes, Message};
use std::net::SocketAddr;
use std::fmt;

extern crate rand;
use rand::Rng;

const PACKET_RECV_EVENT: Token = Token(0);

struct Payload<T: AsRef<[u8]>>(T);

impl<T: AsRef<[u8]>> fmt::Display for Payload<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
            .as_ref()
            .iter()
            .map(|elem| write!(f, "{:>3} ", elem))
            .collect()
    }
}

fn msg_send<T: Message>(msg: T, socket: &UdpSocket, addr: &SocketAddr) -> AppResult {
    let mut enc_buf = Vec::new();
    msg.write_to_vec(&mut enc_buf)
        .expect("error serializing response");
    socket.send_to(&enc_buf, addr)?;
    Ok(())
}

pub fn longfi_test(args: cmdline::LongFiTest) -> AppResult {
    let socket = {
        assert_ne!(args.listen_addr_out, args.publish_addr_in);
        debug!("request_addr_out : {}", args.listen_addr_out);
        debug!("response_addr_in: {}", args.publish_addr_in);
        UdpSocket::bind(&args.publish_addr_in)?
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
    let payload: Vec<u8> = (0..args.num_bytes).map(|_| rng.gen::<u8>()).collect();

    println!("sending packet: {:?}", payload);
    let tx_req = msg::LongFiTxUplinkPacket {
        disable_encoding: false,
        disable_fragmentation: args.disable_fragmentation,
        oui: 0xBEEF_FEED,
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
        &args.listen_addr_out,
    )?;

    let mut events = Events::with_capacity(128);
    loop {
        poll.poll(&mut events, None)
            .expect("Error receiving events from Epoll");

        for event in &events {
            // handle epoll events
            if event.token() == PACKET_RECV_EVENT {
                // packet received from server
                let sz = socket.recv(&mut read_buf)?;

                // parse it into a raw packet
                if let Ok(rx) = parse_from_bytes::<msg::LongFiResp>(&read_buf[..sz]) {
                    if let Some(msg::LongFiResp_oneof_kind::rx(pkt)) = &rx.kind {
                        if let Some(id) = args.filter_device_id {
                            if id != pkt.device_id {
                                break;
                            }
                        }

                        print!(
                            "OUI: {}, device_id: {}, rssi: {}, length: {}, payload: ",
                            pkt.oui,
                            pkt.device_id,
                            pkt.rssi,
                            pkt.payload.len()
                        );

                        println!("{}", Payload(pkt.payload.as_slice()));
                    }
                } else {
                    println!("Failed to parse LongFiResp!");
                }
            }
        }
    }
}