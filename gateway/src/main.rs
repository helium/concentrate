use env_logger;
mod app;
mod cmdline;
use std::process;
use structopt::StructOpt;

fn main() {
    env_logger::init();
    let args = cmdline::Args::from_args();
    match app::go(args.interval) {
        Ok(()) => process::exit(0),
        Err(e) => {
            eprintln!("ERROR: {}", e);
            process::exit(1);
        }
    }
}
