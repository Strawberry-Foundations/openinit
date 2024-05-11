use std::fs::File;
use std::io::Write;

use eyre::Result;

use crate::core::err::InitError;
use crate::colors::{BOLD, C_RESET, GRAY, GREEN};
use crate::daemon::system::OpenDaemon;

#[derive(Default)]
pub struct OpenInit {
    daemon: OpenDaemon
}

impl OpenInit {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn is_running(&self) -> Result<(), InitError> {
        match std::fs::metadata("/run/initd.pid") {
            Ok(_) => {
                Err(InitError::AlreadyRunning)
            },
            Err(_) => {
                Ok(())
            }
        }
    }

    pub fn generate_pid_file(&self, pid: i32) -> Result<(), InitError> {
        let mut file = match File::create("/run/initd.pid") {
            Ok(res) => res,
            Err(_) => return Err(InitError::PidError)
        };

        match file.write_all(pid.to_string().as_bytes()) {
            Ok(res) => res,
            Err(_) => return Err(InitError::PidError)
        };

        Ok(())
    }

    pub fn start(&mut self, daemon: OpenDaemon) -> Result<(), InitError> {
        self.is_running()?;
        self.daemon = daemon;

        self.generate_pid_file(1)?;

        let version = match std::fs::read_to_string("/proc/version") {
            Ok(content) => {
                let parts: Vec<&str> = content.trim().split(' ').collect();

                match parts.get(2) {
                    Some(part) => part.to_string(),
                    None => String::new()
                }
            },
            Err(_) => String::new()
        };

        println!("{BOLD}{GREEN}*{C_RESET} OpenInit is starting Linux {GRAY}{BOLD}{version}{C_RESET}\n");

        self.daemon.configure()?;
        self.daemon.start()?;

        Ok(())
    }

    pub fn stop(&self) -> ! {
        std::fs::remove_file("/run/initd.pid").unwrap();
        std::process::exit(1)
    }
}