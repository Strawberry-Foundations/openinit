use std::io::{stdout, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread::spawn;

use eyre::Result;

use crate::core::err::InitError;
use crate::core::target::{PostTarget, Target};
use crate::core::log::{log_fail, log_startup, log_success};
use crate::daemon::service::OpenService;
use crate::util::{delete_last_line, get_os_name};
use crate::colors::{BOLD, C_RESET, GRAY, GREEN, RESET};


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
            let target = Target::new(service);

            if target.post == PostTarget::Shell {
                continue
            }
            
            log_startup(service);

            let command: Vec<String> = service.service.command.split_whitespace().map(String::from).collect();

            if let Some((cmd, args)) = command.split_first() {
                if target.post == PostTarget::Loop {
                    let (tx, rx) = mpsc::channel::<bool>();

                    let cmd = cmd.clone();
                    let args_cloned = args.to_owned().clone();
                    let service_cloned = service.clone();

                    let handle = std::thread::Builder::new().name("service_thread".to_string()).spawn(move || {
                        match Command::new(cmd)
                            .args(args_cloned)
                            .stdout(Stdio::null())
                            .stderr(Stdio::piped())
                            .spawn() {
                            Ok(res) => {
                                tx.send(true).unwrap();
                                res
                            }
                            Err(_) => {
                                tx.send(false).unwrap();

                                Command::new("false").spawn().unwrap()
                            }
                        }
                    });

                    match rx.recv().unwrap() {
                        true => {
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
                        },
                        false => {
                            delete_last_line();
                            log_fail(&service_cloned);
                        }
                    };
                }
                else {
                    let child = match Command::new(cmd)
                        .args(args)
                        .stdout(Stdio::null())
                        .stderr(Stdio::piped())
                        .status() {
                        Ok(res) => { res }
                        Err(_) => {
                            delete_last_line();
                            log_fail(service);

                            continue;
                        }
                    };

                    if child.code().unwrap() == 1 {
                        delete_last_line();
                        log_fail(service);
                        
                        continue;
                    }

                    delete_last_line();
                    log_success(service);
                }
            } else {
                delete_last_line();
                log_fail(service);
            }
        }

        let service = &self.services.iter().find(|service| {
            let target = Target::new(service);
            target.post == PostTarget::Shell
        }).unwrap();

        log_startup(service);

        let shell_service = self.services.clone();

        let tty_thread = spawn(move || {
            loop {
                print!("{}[2J", 27 as char);
                print!("{}[H", 27 as char);
                stdout().flush().unwrap();

                println!(
                    "\nWelcome to {}\n",
                    get_os_name()
                );
                
                let result = &shell_service.iter().find(|service| {
                    let target = Target::new(service);
                    target.post == PostTarget::Shell
                });

                match result {
                    Some(service) => {
                        let command: Vec<String> = service.service.command.split_whitespace().map(String::from).collect();

                        if let Some((cmd, args)) = command.split_first() {
                            let mut child = Command::new(cmd)
                                .args(args)
                                .stdin(Stdio::inherit())
                                .stdout(Stdio::inherit())
                                .stderr(Stdio::inherit())
                                .spawn()
                                .unwrap_or_else(|_| {
                                    log_fail(service);
                                    Command::new("/bin/sh")
                                        .stdin(Stdio::inherit())
                                        .stdout(Stdio::inherit())
                                        .stderr(Stdio::inherit())
                                        .spawn().
                                        unwrap_or_else(|_| {
                                            log_fail(service);
                                            std::process::exit(1);
                                        })
                                });

                            let _ = child.wait();
                        }
                    }
                    None => {
                        println!("{BOLD}{GRAY}*{GREEN} Success{C_RESET}   Started service {GRAY}shell{RESET} - {GRAY}System shell{RESET}", );
                        let mut child = Command::new("/bin/sh")
                            .stdin(Stdio::inherit())
                            .stdout(Stdio::inherit())
                            .stderr(Stdio::inherit())
                            .spawn().unwrap();

                        let _ = child.wait();
                    }
                }
            }
        }).join();

        match tty_thread {
            Ok(_) => {
                delete_last_line();
                log_success(service);
            },
            Err(_) => {
                log_fail(service);
                std::process::exit(1);
            }
        }

        /*
        let handle = std::thread::Builder::new().name("shell_thread".to_string()).spawn(move || {

        });

        match handle {
            Ok(a) => {
                delete_last_line();
                dbg!("Ok");

                println!("{:?}", a)
            }
            Err(_) => {
                delete_last_line();
                dbg!("Err");
            }
        } */


        Ok(())
    }
}