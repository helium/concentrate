#![forbid(clippy::panicking_unwrap)]

extern crate byteorder;
extern crate colored;
#[cfg(feature = "log_env")]
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate loragw;
extern crate messages;
extern crate protobuf;
#[macro_use]
extern crate quick_error;
#[cfg(any(feature = "log_env", feature = "log_sys"))]
extern crate log_panics;
extern crate serde;
extern crate structopt;
#[cfg(feature = "log_sys")]
extern crate syslog;
extern crate toml;

mod app;
mod cfg;
mod cmdline;
mod error;

use colored::Colorize;
use error::AppResult;
use std::net::IpAddr;
use std::str::FromStr;
use std::{fs, process};
use structopt::StructOpt;

#[cfg(feature = "log_env")]
fn init_logging() {
    use env_logger::{Builder, Env};
    Builder::from_env(
        Env::new()
            .filter("CONCENTRATE_LOG")
            .write_style("CONCENTRATE_LOG_STYLE"),
    )
    .init();
    log_panics::init();
}

#[cfg(feature = "log_sys")]
fn init_logging() {
    use syslog::*;
    init_unix(Facility::LOG_USER, log::LevelFilter::Debug).unwrap();
    log_panics::init();
}

#[cfg(not(any(feature = "log_env", feature = "log_sys")))]
fn init_logging() {}

fn go(args: cmdline::Args) -> AppResult {
    match args.cmd {
        cmdline::Cmd::Serve { cfg_file } => {
            let cfg = match cfg_file {
                Some(path) => Some(fs::read_to_string(path)?),
                None => None,
            };
            let remote_ip = match IpAddr::from_str(&args.remote_ip) {
                Ok(ip) => Some(ip),
                _ => None,
            };

            app::serve(
                cfg.as_ref().map(std::convert::AsRef::as_ref),
                args.interval,
                args.print_level,
                args.listen_port,
                args.publish_port,
                remote_ip,
            )
        }
        cmdline::Cmd::Listen => app::listen(args.print_level, args.publish_port),
        cmdline::Cmd::LongFi => app::longfi(args.print_level, args.publish_port),
        cmdline::Cmd::Send {
            implicit,
            freq,
            radio,
            power,
            spreading,
            coderate,
            bandwidth,
            payload,
        } => app::send(
            args.print_level,
            args.listen_port,
            args.publish_port,
            implicit,
            freq as u32,
            radio,
            power,
            spreading,
            coderate,
            bandwidth,
            payload,
        ),
        cmdline::Cmd::Bist => app::built_in_self_test(),
    }
}

fn main() {
    //init_logging();
    let args = cmdline::Args::from_args();
    match go(args) {
        Ok(()) => process::exit(0),
        Err(e) => {
            eprintln!("{} {}", "error:".red().bold(), e);
            process::exit(1);
        }
    }
}
