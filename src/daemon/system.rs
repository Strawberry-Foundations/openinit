use std::process::{Command, Stdio};
use eyre::Result;

use crate::daemon::service::OpenService;
use crate::core::err::InitError;
use crate::util::delete_last_line;

use crate::colors::{BOLD, C_RESET, GREEN, RED, YELLOW};

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
            println!(
                "[{BOLD}{YELLOW} Startup {C_RESET}]  Starting service {} - {}",
                service.service.name, service.service.description
            );

            let command: Vec<&str> = service.service.command.split_whitespace().collect();

            if let Some((cmd, args)) = command.split_first() {

                let mut child = match Command::new(cmd)
                    .args(args)
                    .stdout(Stdio::null())
                    .stderr(Stdio::piped())
                    .spawn() {
                    Ok(res) => { res }
                    Err(_) => {
                        eprintln!(
                            "[{BOLD}{RED}   Fail  {C_RESET}]  Invalid command for service {}",
                            service.service.name
                        );
                        
                        Command::new("").spawn().unwrap()
                    }
                };

                let _ = child.wait().unwrap();

                delete_last_line();
                println!(
                    "[{BOLD}{GREEN} Service {C_RESET}]  Started service {} - {}",
                    service.service.name, service.service.description
                );
            } else {
                println!(
                    "[{BOLD}{RED}   Fail  {C_RESET}]  Invalid command for service {}",
                    service.service.name
                );
            }
    /*
            use std::process::{Command, Stdio};

            let mut child = Command::new("sh")
                .arg("-c")
                .arg("sleep 5 && echo Hello from child process")
                .stdout(Stdio::piped())
                .spawn()
                .expect("failed to execute command");

            // Auf das Beenden des Child-Prozesses warten
            let output = child.wait().expect("failed to wait on child");

            println!("Child process exited with: {:?}", output); */
        }
        Ok(())
    }
}