use std::error::Error;
use std::io::{BufReader, Read};
use std::process::exit;

extern crate loragw;

fn main() {
    let (_, tty) = loragw::GPS::open("/dev/ttyS0", 9600, None).unwrap();
    let rdr = BufReader::new(tty);
    let mut deframer = loragw::Deframer::new();
    for byte in rdr.bytes() {
        match byte {
            Ok(byte) => {
                if let Some(msg) = deframer.push(byte) {
                    println!();
                    println!("{:?}", msg);
                    exit(0);
                }
            }
            Err(e) => eprintln!("read error {}", e.description()),
        }
    }
}
