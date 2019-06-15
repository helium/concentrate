use std::path::PathBuf;
use structopt::clap::ArgGroup;
use structopt::StructOpt;

fn chan_or_freq() -> ArgGroup<'static> {
    ArgGroup::with_name("chan_or_freq").required(true)
}

#[derive(StructOpt)]
pub enum Cmd {
    /// Operate as a server between concentrator hardware and UDP clients.
    #[structopt(name = "serve")]
    Serve {
        #[structopt(short = "c", long = "config", parse(from_os_str))]
        cfg_file: Option<PathBuf>,
    },

    /// Operate as a consumer of another instance running as the
    /// server. This mode is primarily meant for debugging and
    /// printing [de]serialized packets.
    #[structopt(name = "listen")]
    Listen,

    /// Transmit a packet using provided string as payload. Similar to
    /// 'listen', requires another instance running in
    /// 'serve' mode.
    #[structopt(name = "send")]
    #[structopt(raw(group = "chan_or_freq()"))]
    Send {
        /// Transmit with implicit header.
        #[structopt(short = "i", long = "implicit")]
        implicit: bool,

        /// Frequency to transmit packet on.
        #[structopt(value_name = "Hz", short = "f", long = "freq", group = "chan_or_freq")]
        freq: Option<f64>,

        /// Channel to transmit packet on [0,7].
        #[structopt(
            value_name = "channel",
            short = "c",
            long = "channel",
            group = "chan_or_freq"
        )]
        chan: Option<u32>,

        /// Radio [0,1] to transmit on.
        #[structopt(short = "r", long = "radio")]
        radio: u8,

        /// Transmit power.
        #[structopt(value_name = "dBm", short = "p", long = "power", default_value = "0")]
        power: i8,

        /// Spreading factor [7,8,9,10,11,12].
        #[structopt(
            value_name = "SF",
            short = "s",
            long = "spreading",
            default_value = "7"
        )]
        spreading: u8,

        /// Coderate [5,6,7,8]. Actual coderate is 4/[VALUE].
        #[structopt(long = "coderate", default_value = "5")]
        coderate: u8,

        /// Bandwidth [7800,15600,31200,62500,125000,250000,500000]
        #[structopt(
            value_name = "Hz",
            short = "b",
            long = "bandwidth",
            default_value = "125000"
        )]
        bandwidth: u32,

        /// String payload.
        payload: Option<String>,
    },
}

#[derive(StructOpt)]
pub struct Args {
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

    /// Publish port. UDP port number to publish received packets to.
    #[structopt(
        value_name = "PORT",
        short = "u",
        long = "publish",
        default_value = "31337"
    )]
    pub publish_port: u16,

    /// Listen port. UDP port number to listen for send packet TX requests.
    #[structopt(
        value_name = "PORT",
        short = "l",
        long = "listen",
        default_value = "31338"
    )]
    pub listen_port: u16,

    #[structopt(subcommand)]
    pub cmd: Cmd,
}
