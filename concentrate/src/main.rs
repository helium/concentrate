extern crate colored;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate loragw;
extern crate messages;
extern crate protobuf;
#[macro_use]
extern crate quick_error;
extern crate serde;
extern crate structopt;
extern crate toml;

mod app;
mod cfg;
mod cmdline;
mod error;

use colored::Colorize;
use env_logger::{Builder, Env};
use error::AppResult;
use std::{fs, process};
use structopt::StructOpt;

fn go(args: cmdline::Args) -> AppResult {
    match args.cmd {
        cmdline::Cmd::Serve { cfg_file } => {
            let cfg = match cfg_file {
                Some(path) => Some(fs::read_to_string(path)?),
                None => None,
            };
            app::serve(
                cfg.as_ref().map(std::convert::AsRef::as_ref),
                args.interval,
                args.print_level,
                args.listen_port,
                args.publish_port,
            )
        }
        cmdline::Cmd::Listen => app::listen(args.print_level, args.publish_port),
        cmdline::Cmd::Send {
            freq,
            radio,
            power,
            spreading,
            coderate,
            bandwidth,
            payload,
        } => app::send(
            args.listen_port,
            freq,
            radio,
            power,
            spreading,
            coderate,
            bandwidth,
            payload,
        ),
    }
}

fn main() {
    Builder::from_env(
        Env::new()
            .filter("CONCENTRATE_LOG")
            .write_style("CONCENTRATE_LOG_STYLE"),
    )
    .init();
    let args = cmdline::Args::from_args();
    match go(args) {
        Ok(()) => process::exit(0),
        Err(e) => {
            eprintln!("{} {}", "error:".red().bold(), e);
            process::exit(1);
        }
    }
}
