use env_logger::{Builder, Env};
mod app;
mod cmdline;
use std::process;
use structopt::StructOpt;
mod error;

fn main() {
    Builder::from_env(Env::new().filter("CONCENTRATE_LOG").write_style("CONCENTRATE_LOG_STYLE")).init();
    let args = cmdline::Args::from_args();
    match app::go(
        args.interval,
        args.print_level,
        args.listen_port,
        args.publish_port,
    ) {
        Ok(()) => process::exit(0),
        Err(e) => {
            eprintln!("ERROR: {}", e);
            process::exit(1);
        }
    }
}
