use crate::colors::{BOLD, C_RESET, GRAY, GREEN, RED, RESET, YELLOW};
use crate::daemon::service::OpenService;

pub fn log_fail(service: &OpenService) {
    println!(
        "{BOLD}{GRAY}*{RED} Fail   {C_RESET}   Failed to start service {GRAY}{}{RESET}",
        service.service.name
    );
}

pub fn log_success(service: &OpenService) {
    println!(
        "{BOLD}{GRAY}*{GREEN} Success{C_RESET}   Started service {GRAY}{}{RESET} - {GRAY}{}{RESET}",
        service.service.name, service.service.description
    );
}

pub fn log_startup(service: &OpenService) {
    println!(
        "{BOLD}{GRAY}*{YELLOW} Wait   {C_RESET}   Starting service {GRAY}{}{RESET} - {GRAY}{}{RESET}",
        service.service.name, service.service.description
    );
}