use crate::daemon::service::OpenService;
use crate::colors::{BLUE, BOLD, C_RESET, GRAY, GREEN, RED, RESET, YELLOW};

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

pub fn log_info(service: &OpenService, info_message: impl ToString) {
    println!(
        "{BOLD}{GRAY}*{BLUE} Info   {C_RESET}   Info from {GRAY}{}{RESET} - {GRAY}{}{RESET}",
        service.service.name, info_message.to_string()
    );
}