use crate::error::AppResult;
use longfi_hotspot::{LongFi, LongFiResponse};
use messages as msg;
use mio::net::UdpSocket;
use mio::{Events, Poll, PollOpt, Ready, Token};
use mio_extras::timer::{Timeout, Timer};
use protobuf::{parse_from_bytes, Message};
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

const PACKET_RECV_EVENT: Token = Token(0);
const PACKET_TIMEOUT: Token = Token(1);
const PACKET_SEND_EVENT: Token = Token(2);

fn msg_send<T: Message>(msg: T, socket: &UdpSocket, addr: &SocketAddr) -> AppResult {
    let mut enc_buf = Vec::new();
    msg.write_to_vec(&mut enc_buf)
        .expect("error serializing response");
    socket.send_to(&enc_buf, addr)?;
    Ok(())
}

pub fn longfi(
    _print_level: u8,
    in_port: u16,
    out_port: u16,
    radio_ip: Option<IpAddr>,
    longfi_out_port: u16,
    longfi_in_port: u16,
    _longfi_ip: Option<IpAddr>,
) -> AppResult {
    let (socket, addr_out, longfi_socket, longfi_addr_out) = {
        let addr_in;
        let addr_out;

        if let Some(remote_ip) = radio_ip {
            addr_in = SocketAddr::from(([0, 0, 0, 0], in_port));
            addr_out = SocketAddr::from((remote_ip, out_port));
        } else {
            addr_in = SocketAddr::from(([127, 0, 0, 1], in_port));
            addr_out = SocketAddr::from(([127, 0, 0, 1], out_port));
        }

        let longfi_addr_in = SocketAddr::from(([127, 0, 0, 1], longfi_in_port));
        let longfi_addr_out = SocketAddr::from(([127, 0, 0, 1], longfi_out_port));

        assert_ne!(addr_in, addr_out);
        debug!("radio_addr_in : {}", addr_in);
        debug!("radio_addr_out: {}", addr_out);

        debug!("longfi_addr_in : {}", longfi_addr_in);
        debug!("longfi_addr_out: {}", longfi_addr_out);
        (
            UdpSocket::bind(&addr_in)?,
            addr_out,
            UdpSocket::bind(&longfi_addr_in)?,
            longfi_addr_out,
        )
    };

    let mut read_buf = [0; 1024];
    let mut longfi = LongFi::new();

    let poll = Poll::new().expect("Error initializing poll object");
    let mut timer: Timer<usize> = Timer::default();
    let mut timeouts: [Option<Timeout>; 256] = array_of_none_256!();
    poll.register(&timer, PACKET_TIMEOUT, Ready::readable(), PollOpt::edge())
        .unwrap();

    poll.register(
        &socket,
        PACKET_RECV_EVENT,
        Ready::readable(),
        PollOpt::edge(),
    )
    .unwrap();

    poll.register(
        &longfi_socket,
        PACKET_SEND_EVENT,
        Ready::readable(),
        PollOpt::edge(),
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
                    let sz = socket.recv(&mut read_buf)?;
                    // parse it into a raw packet
                    match parse_from_bytes::<msg::RadioResp>(&read_buf[..sz]) {
                        // feed raw packet to longfi parser
                        Ok(resp) => longfi.handle_response(&resp),
                        Err(e) => {
                            error!("{:?}", e);
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
                            error!("{:?}", e);
                            None
                        }
                    }
                }
                PACKET_TIMEOUT => {
                    // packet has timed out, so cancel it
                    if let Some(index) = timer.poll() {
                        longfi.parser_timeout(index)
                    } else {
                        None
                    }
                }
                _ => None,
            };

            // if there was a response, deal with it
            if let Some(response) = maybe_response {
                match response {
                    LongFiResponse::PktRx(pkt) => {
                        // packet received, cancel the timeout
                        if let Some(timeout) = timeouts[pkt.packet_id as usize].take() {
                            timer.cancel_timeout(&timeout);
                        }

                        let quality_str = pkt.get_quality_string();

                        let rx_packet: msg::LongFiRxPacket = pkt.into();
                        // only forward a packet to client if CRC pass on every fragment
                        if rx_packet.crc_check {
                            // transform it into a UDP msg for client
                            debug!("Packet received, sending to client: {:?}", rx_packet);

                            let resp = msg::LongFiResp {
                                id: 0,
                                kind: Some(msg::LongFiResp_oneof_kind::rx(rx_packet)),
                                ..Default::default()
                            };
                            // send to client
                            msg_send(resp, &longfi_socket, &longfi_addr_out)?;
                        } else {
                            // transform it into a UDP msg for client
                            debug!(
                                "Packet received, with CRC error ({}), dropping: {:?}",
                                quality_str, rx_packet
                            );
                        }
                    }
                    // the parser got a header fragment and will continue parsing the packet
                    // NOTE: there is a known bug here where a new timeout configuration writes over the previous one
                    LongFiResponse::FragmentedPacketBegin(index) => {
                        timeouts[index] = Some(timer.set_timeout(Duration::new(4, 0), index));
                    }
                    LongFiResponse::RadioReq(msg) => {
                        debug!("[LongFi] Sending fragment to radio via UDP");
                        msg_send(msg, &socket, &addr_out)?;
                    }
                    LongFiResponse::ClientResp(resp) => {
                        msg_send(resp, &longfi_socket, &longfi_addr_out)?;
                    }
                }
            }
        }
    }
}
