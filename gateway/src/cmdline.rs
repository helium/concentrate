use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Args {
    /// Polling interval. How often to poll concentrator's FIFO for received packets.
    #[structopt(
        name = "MILLISECONDS",
        short = "I",
        long = "interval",
        default_value = "100"
    )]
    pub interval: u64,
}
