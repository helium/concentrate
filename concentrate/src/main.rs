use env_logger::{Builder, Env};
mod app;
mod cmdline;
use std::process;
use structopt::StructOpt;
mod error;

fn main() {
    Builder::from_env(
        Env::new()
            .filter("CONCENTRATE_LOG")
            .write_style("CONCENTRATE_LOG_STYLE"),
    )
    .init();
    let args = cmdline::Args::from_args();
    let res = match args.cmd {
        cmdline::Cmd::Serve => app::serve(
            args.interval,
            args.print_level,
            args.listen_port,
            args.publish_port,
        ),
        cmdline::Cmd::Listen => app::listen(args.print_level, args.publish_port),
    };
    match res {
        Ok(()) => process::exit(0),
        Err(e) => {
            eprintln!("ERROR: {}", e);
            process::exit(1);
        }
    }
}
