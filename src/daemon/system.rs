use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread::spawn;
use eyre::Result;

use crate::daemon::service::OpenService;
use crate::core::err::InitError;
use crate::util::delete_last_line;

use crate::core::log::{log_fail, log_startup, log_success};
use crate::core::target::{PostTarget, Target};

#[derive(Default)]
pub struct OpenDaemon {
    pub services: Vec<OpenService>
}

impl OpenDaemon {
    pub fn new() -> Self {
        Self {
            services: vec![]
        }
    }

    pub fn configure(&mut self) -> Result<(), InitError> {
        // let service_folder = "./openinit.d";
        let service_folder = "/etc/openinit.d";

        let services = match std::fs::read_dir(service_folder) {
            Ok(res) => res,
            Err(_) => return Err(InitError::ServiceDirectoryNotFound)
        };

        // todo: remove unwrap
        for service in services {
            let entry = service.unwrap();
            let service_path = entry.path();

            if service_path.is_file() && (service_path.extension().unwrap_or_default() == "yaml" || service_path.extension().unwrap_or_default() == "yml") {
                let service_content = std::fs::read_to_string(&service_path).unwrap();
                let data: OpenService = serde_yaml::from_str(&service_content).unwrap();

                self.services.push(data);
            }
        }

        Ok(())
    }

    pub fn start(&mut self) -> Result<(), InitError> {
        for service in &self.services {
            log_startup(service);

            let command: Vec<&str> = service.service.command.split_whitespace().collect();

            if let Some((cmd, args)) = command.split_first() {
                let target = Target::new(service);
                
                if target.post == PostTarget::Loop {
                    let cmd = cmd.to_string();
                    let cmd_clone = cmd.as_str().to_owned();
                    
                    let handle = std::thread::Builder::new().name("service_thread".to_string()).spawn(move || {
                        match Command::new(cmd_clone)
                            .args(["&b"])
                            .stdout(Stdio::null())
                            .stderr(Stdio::piped())
                            .spawn() {
                            Ok(res) => { res }
                            Err(_) => {
                                // Handle error
                                delete_last_line();
                                Command::new("false").spawn().unwrap()
                            }
                        }
                    });

                    match handle {
                        Ok(_) => {
                            delete_last_line();
                            log_success(service);
                        }
                        Err(_) => {
                            delete_last_line();
                            log_fail(service);
                        }
                    }
                }
                else {
                    let mut child = match Command::new(cmd)
                        .args(args)
                        .stdout(Stdio::null())
                        .stderr(Stdio::piped())
                        .spawn() {
                        Ok(res) => { res }
                        Err(_) => {
                            delete_last_line();
                            log_fail(service);

                            continue;
                        }
                    };
                    
                    let _ = child.wait().unwrap();
                    
                    delete_last_line();
                    log_success(service);
                }
            } else {
                delete_last_line();
                log_fail(service);
            }
        }
        Ok(())
    }
}