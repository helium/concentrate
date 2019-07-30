use std::{net::IpAddr, path::PathBuf};
use structopt::{clap, StructOpt};

#[derive(Debug, StructOpt)]
pub enum Cmd {
    /// Built In Self Test. Configures, starts, then immediately stops
    /// concentrator. Returns 1 on hardware failure.
    #[structopt(name = "bist", raw(setting = "clap::AppSettings::Hidden"))]
    Bist,

    /// Operate as a consumer of another instance running as the
    /// server. This mode is primarily meant for debugging and
    /// printing [de]serialized packets.
    #[structopt(name = "listen")]
    Listen(Listen),

    /// Operate as a LongFi server. Requires separate `concentrate
    /// serve` instance.
    #[structopt(name = "longfi")]
    LongFi(LongFi),

    /// This mode is primarily meant for testing and debugging LongFi
    /// device code.
    #[structopt(name = "longfi-test", raw(setting = "clap::AppSettings::Hidden"))]
    LongFiTest(LongFiTest),

    /// Transmit a packet using provided string as payload. Similar to
    /// 'listen', requires another instance running in
    /// 'serve' mode.
    #[structopt(name = "send")]
    Send(Send),

    /// Operate as a server between concentrator hardware and UDP clients.
    #[structopt(name = "serve")]
    Serve(Serve),
}

#[derive(Debug, StructOpt)]
pub struct Listen {
    /// Print packets. `-p` will print on a single line, and `-pp` will pretty-print over several.
    #[structopt(short = "p", parse(from_occurrences))]
    pub print_level: u8,

    /// Listen port. UDP port number to listen for received packets on.
    #[structopt(
        value_name = "PORT",
        short = "l",
        long = "listen",
        default_value = "31337"
    )]
    pub listen_port: u16,
}

#[derive(Debug, StructOpt)]
pub struct LongFi {
    /// UDP port number to listen for LongFi transmit requests.
    #[structopt(value_name = "PORT", long = "listen", default_value = "31341")]
    pub longfi_listen_port: u16,

    /// UDP port to publish received LongFi packets to.
    #[structopt(value_name = "PORT", long = "publish", default_value = "31340")]
    pub longfi_publish_port: u16,

    /// Remote IP to send radio transmit requests to.
    #[structopt(value_name = "ADDR", long = "request-addr")]
    pub radio_request_addr: Option<IpAddr>,

    /// UDP port to send radio transmit requests to.
    #[structopt(value_name = "PORT", long = "request-port", default_value = "31338")]
    pub radio_request_port: u16,

    /// UDP port number to listen for radio responses.
    #[structopt(value_name = "PORT", long = "response", default_value = "31337")]
    pub radio_response_port: u16,
}

#[derive(Debug, StructOpt)]
pub struct LongFiTest {
    /// Remote IP to send LongFi transmit requests to.
    #[structopt(value_name = "ADDR", long = "request-addr")]
    pub longfi_request_addr: Option<IpAddr>,

    /// UDP port number to send LongFi transmit requests to.
    #[structopt(value_name = "PORT", long = "request", default_value = "31341")]
    pub longfi_request_port: u16,

    /// UDP port to listen for LongFi responses on.
    #[structopt(value_name = "PORT", long = "response", default_value = "31340")]
    pub longfi_response_port: u16,
}

#[derive(Debug, StructOpt)]
pub struct Send {
    /// Request port. UDP port number to send our TX request to.
    #[structopt(value_name = "PORT", long = "request", default_value = "31338")]
    pub request_port: u16,

    /// Response port. UDP port number to listen for a response to our
    /// TX request.
    #[structopt(value_name = "PORT", long = "response", default_value = "31337")]
    pub response_port: u16,

    /// Print packets. `-p` will print on a single line, and `-pp` will pretty-print over several.
    #[structopt(short = "p", parse(from_occurrences))]
    pub print_level: u8,

    /// Transmit with implicit header.
    #[structopt(short = "i", long = "implicit")]
    pub implicit: bool,

    /// Frequency to transmit packet on.
    #[structopt(value_name = "Hz", short = "f", long = "freq")]
    pub freq: f64,

    /// Radio [0,1] to transmit on.
    #[structopt(short = "r", long = "radio")]
    pub radio: u8,

    /// Transmit power.
    #[structopt(value_name = "dBm", short = "o", long = "power", default_value = "0")]
    pub power: i8,

    /// Spreading factor [7,8,9,10,11,12].
    #[structopt(
        value_name = "SF",
        short = "s",
        long = "spreading",
        default_value = "7"
    )]
    pub spreading: u8,

    /// Coderate [5,6,7,8]. Actual coderate is 4/[VALUE].
    #[structopt(short = "c", long = "coderate", default_value = "5")]
    pub coderate: u8,

    /// Bandwidth [7800,15600,31200,62500,125000,250000,500000]
    #[structopt(
        value_name = "Hz",
        short = "b",
        long = "bandwidth",
        default_value = "125000"
    )]
    pub bandwidth: u32,

    /// String payload.
    pub payload: Option<String>,
}

#[derive(Debug, StructOpt)]
pub struct Serve {
    /// Polling interval. How often to poll concentrator's FIFO for received packets.
    #[structopt(
        value_name = "MILLISECONDS",
        short = "I",
        long = "interval",
        default_value = "100"
    )]
    pub interval: u64,

    /// Print packets. `-p` will print on a single line, and `-pp` will pretty-print over several.
    #[structopt(short = "p", parse(from_occurrences))]
    pub print_level: u8,

    #[structopt(short = "c", long = "config", parse(from_os_str))]
    pub cfg_file: Option<PathBuf>,

    /// Listen port. UDP port number to listen for send packet TX requests.
    #[structopt(
        value_name = "PORT",
        short = "l",
        long = "listen",
        default_value = "31338"
    )]
    pub listen_port: u16,

    /// Publish port. UDP port number to publish received packets to.
    #[structopt(
        value_name = "PORT",
        short = "u",
        long = "publish",
        default_value = "31337"
    )]
    pub publish_port: u16,

    /// Remote IP for listening.
    #[structopt(value_name = "REMOTE_IP", short = "r", long = "remote-ip")]
    pub remote_ip: Option<IpAddr>,
}
