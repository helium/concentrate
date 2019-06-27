use std::error::Error;
use std::ffi::CString;
use std::io::{BufReader, Read};
extern crate loragw;

#[derive(Debug)]
enum Frame {
    Nmea(CString),
    Ublox(Vec<u8>),
}

#[derive(Debug)]
enum Collect {
    Nmea(Vec<u8>),
    Ublox(Vec<u8>),
}

#[derive(Debug)]
enum State {
    Sync,
    Collect(Collect),
}

#[derive(Debug)]
struct Deframer {
    state: State,
}

impl Deframer {
    pub fn new() -> Self {
        Deframer { state: State::Sync }
    }

    pub fn push(&mut self, b: u8) -> Option<Frame> {
        const NMEA_SYNC_CHAR: u8 = b'$';
        const NMEA_LF_CHAR: u8 = b'\n';
        const UBLOX_SYNC_CHAR: u8 = 0xB5;

        let (frame, next_state) = match (&mut self.state, b) {
            (State::Sync, NMEA_SYNC_CHAR) => (None, Some(State::Collect(Collect::Nmea(vec![b])))),
            (State::Sync, UBLOX_SYNC_CHAR) => (None, Some(State::Collect(Collect::Ublox(vec![b])))),
            (State::Sync, _) => (None, None),
            (State::Collect(Collect::Nmea(payload)), NMEA_LF_CHAR) => {
                match CString::new(payload.as_slice()) {
                    Ok(msg) => (Some(Frame::Nmea(msg)), (Some(State::Sync))),
                    Err(_) => (None, (Some(State::Sync))),
                }
            }
            (State::Collect(Collect::Nmea(payload)), b) => {
                payload.push(b);
                (None, None)
            }
            (State::Collect(Collect::Ublox(payload)), NMEA_SYNC_CHAR) => (
                Some(Frame::Ublox(payload.to_vec())),
                Some(State::Collect(Collect::Nmea(vec![b]))),
            ),
            (State::Collect(Collect::Ublox(payload)), UBLOX_SYNC_CHAR) => (
                Some(Frame::Ublox(payload.to_vec())),
                Some(State::Collect(Collect::Ublox(vec![b]))),
            ),
            (State::Collect(Collect::Ublox(payload)), b) => {
                payload.push(b);
                (None, None)
            }
        };

        if let Some(next_state) = next_state {
            self.state = next_state;
        }

        frame
    }
}

fn main() {
    let (_gps, tty) = loragw::GPS::open("/dev/ttyS0", 9600).unwrap();
    let rdr = BufReader::new(tty);
    let mut deframer = Deframer::new();
    for b in rdr.bytes() {
        match b {
            Ok(b) => {
                if let Some(f) = deframer.push(b) {
                    println!("{:?}", f);
                }
            }
            Err(e) => eprintln!("read error {}", e.description()),
        }
    }
}
