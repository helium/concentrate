#![forbid(unsafe_code)]

mod app;
mod cfg;
mod cmdline;
mod error;

use crate::error::AppResult;
use env_logger;
use std::{env, process};
use structopt::StructOpt;
use syslog;

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

fn init_logging() {
    const CONCENTRATE_LOG: &str = "CONCENTRATE_LOG";
    if env::var(CONCENTRATE_LOG).is_ok() {
        env_logger::Builder::from_env(
            env_logger::Env::new()
                .filter("CONCENTRATE_LOG")
                .write_style("CONCENTRATE_LOG_STYLE"),
        )
        .init();
    } else {
        syslog::init_unix(syslog::Facility::LOG_USER, log::LevelFilter::Debug).unwrap();
    }
    log_panics::init();
}

fn go(cmd: cmdline::Cmd) -> AppResult {
    use crate::cmdline::Cmd::*;

    match cmd {
        Bist => app::built_in_self_test(),
        Connect => app::connect(),
        Listen(args) => app::listen(args),
        LongFi(args) => app::longfi(args),
        LongFiTest(args) => app::longfi_test(args),
        Send(args) => app::send(args),
        Serve(args) => app::serve(args),
    }
}
