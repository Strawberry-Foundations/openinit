use crate::core::init::OpenInit;
use crate::core::err::InitError;
use crate::daemon::system::OpenDaemon;
use crate::sys::reboot;
use crate::cli::args::{Args, Command};
use crate::cli::help;
use crate::colors::{BOLD, C_RESET, RED};

mod colors;
mod core;
mod daemon;
mod util;
mod sys;
mod cli;

fn main() {
    let args = Args::collect();
    
    let mut openinit = OpenInit::new();
    let opendaemon = OpenDaemon::new();
    
    match args.command {
        Command::Help => help::help(),
        Command::Reboot => {
            reboot::sys_reboot();
        },
        Command::None => {
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
