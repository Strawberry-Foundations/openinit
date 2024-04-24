use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};

use eyre::Result;

use crate::core::err::InitError;
use crate::colors::{BOLD, C_RESET, GREEN};
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

        self.daemon.configure()?;
        self.daemon.start()?;

        println!("{:?}", self.daemon.services);

        Ok(())
    }

    pub fn stop(&self) -> ! {
        std::fs::remove_file("/run/initd.pid").unwrap();
        std::process::exit(1)
    }
}

pub fn init() -> Result<()> {
    /* println!("[{BOLD}{GREEN} Service {C_RESET}]  Setting up hostname");
    let mut child = Command::new("hostname")
        .args(["-F", "/etc/hostname"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    let _ = child.wait(); */



    println!("[{BOLD}{GREEN} Service {C_RESET}]  Starting shell (/bin/sh)");
    let mut child = Command::new("/bin/sh")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    let _ = child.wait();

    Ok(())
}