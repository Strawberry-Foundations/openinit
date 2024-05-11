extern crate libc;

use libc::{kill, pid_t};
use std::process;
use crate::colors::{BOLD, C_RESET, RED, YELLOW};

pub fn panic() {
    let pid: pid_t = 1;

    unsafe {
        let result = kill(pid, libc::SIGKILL);

        if result == -1 {
            eprintln!("{BOLD}{RED}Could not kill init process{C_RESET}");
            process::exit(1);
        } else {
            eprintln!("{BOLD}{YELLOW}Killed init process{C_RESET}");
        }
    }
}
