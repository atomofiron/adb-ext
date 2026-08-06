#[cfg(target_os = "linux")]
pub mod usb_device;
#[cfg(target_os = "linux")]
pub mod permission_resolver;

use crate::core::ext::Rslt;
#[cfg(not(target_os = "linux"))]
use crate::core::utils::strings::LINUX_ONLY;

#[cfg(not(target_os = "linux"))]
pub fn fix_on_linux(_serial: Option<String>) -> Rslt<()> {
    LINUX_ONLY.to_err()
}

#[cfg(target_os = "linux")]
pub fn fix_on_linux(serial: Option<String>) -> Rslt<()> {
    return permission_resolver::fix_permission(serial)
}

#[cfg(not(target_os = "linux"))]
pub fn sudo_fix_on_linux(_serial: Option<String>) -> Rslt<()> {
    LINUX_ONLY.to_err()
}

#[cfg(target_os = "linux")]
pub fn sudo_fix_on_linux(serial: Option<String>) -> Rslt<()> {
    permission_resolver::sudo_fix_permission(serial)
}
