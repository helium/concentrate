use std::error::Error;
use std::io::{BufReader, Read};
use std::process::exit;

extern crate loragw;

fn main() {
    let (mut gps, tty) = loragw::GPS::open("/dev/ttyS0", 9600, None).unwrap();
    let rdr = BufReader::new(tty);
    let mut deframer = loragw::Deframer::new();
    let mut nmea_flag = false;
    let mut ubx_flag = false;
    for byte in rdr.bytes() {
        match byte {
            Ok(byte) => {
                if let Some(msg) = deframer.push(byte) {
                    println!();
                    println!("{:?}", msg);
                   
                    match msg {
                        loragw::Frame::Nmea(_) => nmea_flag = true,
                        loragw::Frame::Ublox(_) => ubx_flag = true,

                    }
                    let _ = gps.parse(msg);

                    if nmea_flag == true && ubx_flag == true {
                        exit(0x00);
                    }
                }
            }
            Err(e) => eprintln!("read error {}", e.description()),
        }
    }
}
