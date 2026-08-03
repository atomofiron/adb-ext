use crate::core::adb_command::AdbArgs;
use crate::core::adb_device::AdbDevice;
use crate::core::r#const::{OFF, ON, SHELL};
use crate::core::selector::{resolve_device, run_adb_for};
use std::process::ExitCode;

const GET_TOUCHES: &str = "settings get system pointer_location";
const PUT_TOUCHES: &str = "settings put system pointer_location";

pub fn is_pointer_on(device: &AdbDevice) -> bool {
    run_adb_for(AdbArgs::run(&[SHELL, GET_TOUCHES]), device.serial.clone()).stdout() == ON
}

pub fn turn_pointer(device: AdbDevice, on: bool) -> ExitCode {
    let value = match on {
        true => ON,
        false => OFF,
    };
    return run_adb_for(AdbArgs::run(&[SHELL, PUT_TOUCHES, value]), device.serial)
        .code
}

pub fn toggle_pointer() -> ExitCode {
    let device = match resolve_device() {
        Ok(device) => device,
        Err(code) => return code,
    };
    let now = is_pointer_on(&device);
    return turn_pointer(device, !now)
}