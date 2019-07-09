use std::error::Error;
use std::io::{BufReader, Read};
extern crate loragw;

fn main() {
    let (mut gps, tty) = loragw::GPS::open("/dev/ttyS0", 9600, None).unwrap();
    let rdr = BufReader::new(tty);
    let mut deframer = loragw::Deframer::new();
    for byte in rdr.bytes() {
        match byte {
            Ok(byte) => {
                if let Some(msg) = deframer.push(byte) {
                    println!();
                    println!("{:?}", msg);
                    gps.parse(msg);
                }
            }
            Err(e) => eprintln!("read error {}", e.description()),
        }
    }
}
