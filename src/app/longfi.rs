use crate::{cmdline, error::AppResult};
use longfi_hotspot::{LongFi, LongFiResponse};
use messages as msg;
use mio::{net::UdpSocket, Events, Poll, PollOpt, Ready, Token};
use protobuf::{parse_from_bytes, Message};
use std::net::SocketAddr;

const PACKET_RECV_EVENT: Token = Token(0);
const PACKET_SEND_EVENT: Token = Token(2);

fn msg_send<T: Message>(msg: T, socket: &UdpSocket, addr: &SocketAddr) -> AppResult {
    let mut enc_buf = Vec::new();
    msg.write_to_vec(&mut enc_buf)
        .expect("error serializing response");
    socket.send_to(&enc_buf, addr)?;
    Ok(())
}

pub fn longfi(args: cmdline::LongFi) -> AppResult {
    let (radio_socket, longfi_socket) = {
        assert_ne!(args.radio_listen_addr_out, args.radio_publish_addr_in);
        log::debug!("radio_listen_addr_out : {}", args.radio_listen_addr_out);
        log::debug!("radio_publish_addr_in: {}", args.radio_publish_addr_in);

        assert_ne!(args.longfi_publish_addr_out, args.longfi_listen_addr_in);
        log::debug!("longfi_publish_addr_out : {}", args.longfi_publish_addr_out);
        log::debug!("longfi_listen_addr_in: {}", args.longfi_listen_addr_in);
        (
            UdpSocket::bind(&args.radio_publish_addr_in)?,
            UdpSocket::bind(&args.longfi_listen_addr_in)?,
        )
    };

    let mut read_buf = [0; 1024];
    let mut longfi = LongFi::new();

    let poll = Poll::new().expect("Error initializing poll object");

    poll.register(
        &radio_socket,
        PACKET_RECV_EVENT,
        Ready::readable(),
        PollOpt::level(),
    )
    .unwrap();

    poll.register(
        &longfi_socket,
        PACKET_SEND_EVENT,
        Ready::readable(),
        PollOpt::level(),
    )
    .unwrap();

    let mut events = Events::with_capacity(128);
    loop {
        poll.poll(&mut events, None)
            .expect("Error receiving events from Epoll");

        for event in &events {
            // handle epoll events
            let maybe_response = match event.token() {
                PACKET_RECV_EVENT => {
                    // packet received from server
                    let sz = radio_socket.recv(&mut read_buf)?;
                    // parse it into a raw packet
                    match parse_from_bytes::<msg::RadioResp>(&read_buf[..sz]) {
                        // feed raw packet to longfi parser
                        Ok(resp) => longfi.handle_response(&resp),
                        Err(e) => {
                            log::error!("{:?}", e);
                            None
                        }
                    }
                }
                PACKET_SEND_EVENT => {
                    // packet received from server
                    let sz = longfi_socket.recv(&mut read_buf)?;
                    // parse it into a raw packet
                    match parse_from_bytes::<msg::LongFiReq>(&read_buf[..sz]) {
                        // feed transmit request to LongFi
                        Ok(req) => longfi.handle_request(&req),
                        Err(e) => {
                            log::error!("{:?}", e);
                            None
                        }
                    }
                }
                _ => None,
            };

            // if there was a response, deal with it
            if let Some(response) = maybe_response {
                match response {
                    LongFiResponse::PktRx(pkt) => {
                        log::debug!("[LongFi][app] Packet received: {:?}", pkt);

                        let rx_packet: msg::LongFiRxPacket = pkt.into();
                        // only forward a packet to client if CRC pass on every fragment
                        if rx_packet.crc_check {
                            // transform it into a UDP msg for client
                            log::debug!("[LongFi][app] Sending to client");

                            let resp = msg::LongFiResp {
                                id: 0,
                                kind: Some(msg::LongFiResp_oneof_kind::rx(rx_packet)),
                                ..Default::default()
                            };
                            // send to client
                            msg_send(resp, &longfi_socket, &args.longfi_publish_addr_out)?;
                        } else {
                            // transform it into a UDP msg for client
                            log::debug!("[LongFi][app] Dropping packet due to CRC error or missing fragments");
                        }
                    }
                    LongFiResponse::RadioReq(msg) => {
                        log::debug!("[LongFi][app] Sending fragment to radio via UDP");
                        msg_send(msg, &radio_socket, &args.radio_listen_addr_out)?;
                    }
                    LongFiResponse::ClientResp(resp) => {
                        msg_send(resp, &longfi_socket, &args.longfi_publish_addr_out)?;
                    }
                }
            }
        }
    }
}
