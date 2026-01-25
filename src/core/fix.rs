#[cfg(target_os = "linux")]
pub mod usb_device;
#[cfg(target_os = "linux")]
pub mod permission_resolver;

use std::process::ExitCode;

#[cfg(not(target_os = "linux"))]
use crate::core::strings::LINUX_ONLY;

#[cfg(not(target_os = "linux"))]
pub fn fix_on_linux(_serial: Option<String>) -> ExitCode {
    LINUX_ONLY.println();
    return ExitCode::FAILURE
}

#[cfg(target_os = "linux")]
pub fn fix_on_linux(serial: Option<String>) -> ExitCode {
    return permission_resolver::fix_permission(serial)
}

#[cfg(not(target_os = "linux"))]
pub fn sudo_fix_on_linux(_serial: Option<String>) -> bool {
    LINUX_ONLY.println();
    return false
}

#[cfg(target_os = "linux")]
pub fn sudo_fix_on_linux(serial: Option<String>) -> bool {
    return permission_resolver::sudo_fix_permission(serial) == ExitCode::SUCCESS
}
