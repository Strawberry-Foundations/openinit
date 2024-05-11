extern crate libc;

use libc::{sync, reboot, RB_POWER_OFF, RB_AUTOBOOT};

pub fn sys_reboot() {
    unsafe {
        sync();
        reboot(RB_AUTOBOOT);
    }
}

pub fn sys_shutdown() {
    unsafe {
        sync();
        reboot(RB_POWER_OFF);
        // shutdown(-1, 0);
    }
}