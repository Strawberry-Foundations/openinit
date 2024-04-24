use crate::core::init::{init, OpenInit};
use crate::daemon::system::OpenDaemon;
use crate::core::err::InitError;

use crate::colors::{BOLD, C_RESET, RED};

mod colors;
mod core;
mod daemon;
mod util;

fn main() {
    let mut openinit = OpenInit::new();
    let opendaemon = OpenDaemon::new();

    openinit.start(opendaemon).unwrap_or_else(|err| {
        eprintln!("{RED}{BOLD}{err}{C_RESET}");

        if err != InitError::AlreadyRunning {
            openinit.stop();
        } else {
            std::process::exit(1);
        }
    });


    match init() {
        Ok(..) => { },
        Err(err) => {
            eprintln!("{RED}{BOLD}Error: {err}{C_RESET}");
            std::process::exit(1);
        }
    }
}
