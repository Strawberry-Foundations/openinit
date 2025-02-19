#![allow(dead_code)]

use crate::cli::args::{Args, Command};
use crate::cli::help;
use crate::core::err::InitError;
use crate::core::init::OpenInit;
use crate::daemon::system::OpenDaemon;
use crate::sys::sysrq;

use crate::colors::{BOLD, C_RESET, RED};

pub const CLEANUP_SCREEN: bool = false;

mod cli;
mod colors;
mod core;
mod daemon;
mod sys;
mod util;

fn main() {
    let args = Args::collect();

    let mut openinit = OpenInit::new();
    let opendaemon = OpenDaemon::new();

    match args.command {
        Command::Help => help::help(),
        Command::Reboot => sysrq::sys_reboot(),
        Command::Shutdown => sysrq::sys_shutdown(),
        Command::Panic => println!("Not implemented"),
        _ => {
            openinit.start(opendaemon).unwrap_or_else(|err| {
                eprintln!("{RED}{BOLD}{err}{C_RESET}");

                if err != InitError::AlreadyRunning {
                    openinit.stop();
                } else {
                    std::process::exit(1);
                }
            });
        }
    }
}
