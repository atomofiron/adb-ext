#[cfg(target_os = "linux")]
pub mod usb_device;
#[cfg(target_os = "linux")]
pub mod permission_resolver;

use crate::core::r#const::ERROR_CODE;
use crate::core::strings::LINUX_ONLY;
#[cfg(target_os = "linux")]
use crate::core::usb_resolver;

pub fn fix_on_linux(_serial: Option<String>) {
    LINUX_ONLY.println();
}

#[cfg(target_os = "linux")]
pub fn fix_on_linux(serial: Option<String>) {
    permission_resolver::fix_permission(serial)
}

pub fn sudo_fix_on_linux(_serial: Option<String>) -> i32 {
    LINUX_ONLY.println();
    return ERROR_CODE
}

#[cfg(target_os = "linux")]
pub fn sudo_fix_on_linux(serial: Option<String>) {
    permission_resolver::sudo_fix_permission(serial)
}
