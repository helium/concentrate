#![forbid(clippy::panicking_unwrap, unsafe_code)]

extern crate byteorder;
#[cfg(feature = "log_env")]
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate longfi_hotspot;
extern crate loragw;
extern crate messages;
extern crate protobuf;
#[macro_use]
extern crate quick_error;
#[cfg(any(feature = "log_env", feature = "log_sys"))]
extern crate log_panics;
extern crate mio;
extern crate mio_extras;
extern crate rand;
extern crate serde;
extern crate structopt;
#[cfg(feature = "log_sys")]
extern crate syslog;
extern crate toml;

mod app;
mod cfg;
mod cmdline;
mod error;

use error::AppResult;
use std::process;
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

fn go(cmd: cmdline::Cmd) -> AppResult {
    use cmdline::Cmd::*;

    match cmd {
        Bist => app::built_in_self_test(),
        Listen(args) => app::listen(args),
        LongFi(args) => app::longfi(args),
        LongFiTest(args) => app::longfi_test(args),
        Send(args) => app::send(args),
        Serve(args) => app::serve(args),
    }
}

fn main() {
    init_logging();
    let cmd = cmdline::Cmd::from_args();
    match go(cmd) {
        Ok(()) => process::exit(0),
        Err(e) => {
            eprintln!("{} {}", "error:", e);
            process::exit(1);
        }
    }
}
