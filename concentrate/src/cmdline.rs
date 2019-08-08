use std::{net::SocketAddr, path::PathBuf};
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
    /// 'listen', requires another instance running in 'serve' mode.
    #[structopt(name = "send")]
    Send(Send),

    /// Operate as a server between concentrator hardware and UDP
    /// clients.
    #[structopt(name = "serve")]
    Serve(Serve),
}

#[derive(Debug, StructOpt)]
pub struct Listen {
    /// Print packets. `-p` will print on a single line, and `-pp`
    /// will pretty-print over several.
    #[structopt(short = "p", parse(from_occurrences))]
    pub print_level: u8,

    /// Address on which to listen for received uplink packets.
    #[structopt(
        value_name = "ADDR",
        short = "l",
        long = "listen",
        default_value = "127.0.0.1:31337"
    )]
    pub listen_addr_in: SocketAddr,
}

#[derive(Debug, StructOpt)]
pub struct LongFi {
    /// Address on which to listen for to-be-transmitted downlink
    /// packets.
    #[structopt(
        value_name = "ADDR",
        long = "longfi-listen",
        default_value = "127.0.0.1:31341"
    )]
    pub longfi_listen_addr_in: SocketAddr,

    /// Address to publish received LongFi packets to.
    #[structopt(
        value_name = "ADDR",
        long = "longfi-publish",
        default_value = "127.0.0.1:31340"
    )]
    pub longfi_publish_addr_out: SocketAddr,

    /// Address to publish raw to-be-transmitted LoRa packets to.
    #[structopt(
        value_name = "ADDR",
        long = "radio-listen",
        default_value = "127.0.0.1:31338"
    )]
    pub radio_listen_addr_out: SocketAddr,

    /// Address on which to listen for raw uplink packets.
    #[structopt(
        value_name = "ADDR",
        long = "radio-publish",
        default_value = "127.0.0.1:31337"
    )]
    pub radio_publish_addr_in: SocketAddr,
}

#[derive(Debug, StructOpt)]
pub struct LongFiTest {
    /// Address to send LongFi requests to.
    #[structopt(
        value_name = "ADDR",
        long = "listen",
        default_value = "127.0.0.1:31341"
    )]
    pub listen_addr_out: SocketAddr,

    /// Address on which to listen for LongFi responses.
    #[structopt(
        value_name = "ADDR",
        long = "publish",
        default_value = "127.0.0.1:31340"
    )]
    pub publish_addr_in: SocketAddr,

    /// number of bytes to transmit
    #[structopt(
        value_name = "NUM_BYTES",
        short = "n",
        long = "num-bytes",
        default_value = "90"
    )]
    pub num_bytes: usize,

    /// disable fragmentation, making it so that bytes are all send monolithically
    #[structopt(short = "d", long = "disable-frag")]
    pub disable_fragmentation: bool,

    /// parse cargo fields and display nicely
    #[structopt(short = "p", long = "parse-cargo")]
    pub parse_cargo: bool,

    /// set device id to filter by
    #[structopt(short = "f", long = "filter-device-id")]
    pub filter_device_id: Option<u32>,
}

#[derive(Debug, StructOpt)]
pub struct Send {
    /// Address to send raw LoRa packets to.
    #[structopt(
        value_name = "ADDR",
        long = "listen",
        default_value = "127.0.0.1:31338"
    )]
    pub listen_addr_out: SocketAddr,

    /// Address on which to listen for transmit responses.
    #[structopt(
        value_name = "ADDR",
        long = "publish",
        default_value = "127.0.0.1:31337"
    )]
    pub publish_addr_in: SocketAddr,

    /// Print packets. `-p` will print on a single line, and `-pp`
    /// will pretty-print over several.
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
    /// Polling interval. How often to poll concentrator's FIFO for
    /// received packets.
    #[structopt(
        value_name = "MILLISECONDS",
        short = "I",
        long = "interval",
        default_value = "10"
    )]
    pub interval: u64,

    /// Print packets. `-p` will print on a single line, and `-pp`
    /// will pretty-print over several.
    #[structopt(short = "p", parse(from_occurrences))]
    pub print_level: u8,

    #[structopt(short = "c", long = "config", parse(from_os_str))]
    pub cfg_file: Option<PathBuf>,

    /// Address on which to listen for requests.
    #[structopt(
        value_name = "ADDR",
        short = "l",
        long = "listen",
        default_value = "127.0.0.1:31338"
    )]
    pub listen_addr_in: SocketAddr,

    /// Address to send responses and received uplink packets to.
    #[structopt(
        value_name = "ADDR",
        short = "u",
        long = "publish",
        default_value = "127.0.0.1:31337"
    )]
    pub publish_addr_out: SocketAddr,
}
