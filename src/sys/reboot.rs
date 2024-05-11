extern crate libc;

use libc::{sync, reboot, RB_POWER_OFF};

pub fn sys_reboot() {
    unsafe {
        sync();
        reboot(RB_POWER_OFF);
    }
}
