use crate::colors::{C_RESET, GREEN, BOLD, UNDERLINE, CYAN, RESET, WHITE, RED, MAGENTA};

pub fn help() {
    println!("\
{BOLD}{CYAN}{UNDERLINE}OpenInit v{}{C_RESET}\n\
{GREEN}{BOLD}Usage:{RESET} {WHITE}openinit {CYAN}[command] {RED}[<options>]{C_RESET}\n\n\
{MAGENTA}{BOLD}Commands:{C_RESET}
    {CYAN}{BOLD}help:{C_RESET} Prints this message
    {CYAN}{BOLD}reboot:{C_RESET} Reboots the system

", env!("CARGO_PKG_VERSION"));
    std::process::exit(0);
}

/* 
    {CYAN}{BOLD}local <port>:{C_RESET} Starts a local proxy to the remote server
     {BOLD}↳ {MAGENTA}Options:{C_RESET}
            {CYAN}{BOLD}-u, --use <server>{C_RESET}      Select your target server for tunneling your traffic
            {CYAN}{BOLD}-l, --local-host <host>{C_RESET} The address to expose                 {GREEN}{BOLD}[default: localhost]{C_RESET}
            {CYAN}{BOLD}-sp, --static-port{C_RESET}      Static port forwarding (whitelist)    {GREEN}{BOLD}[optional]{C_RESET}
 */