use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Cmd {
    /// Operate as a server between concentrator hardware and UDP clients.
    #[structopt(name = "serve")]
    Serve,

    /// Operate as a consumer of another instance running as the
    /// server. This mode is primarily meant for debugging and
    /// printing [de]serialized packets.
    #[structopt(name = "listen")]
    Listen,
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
