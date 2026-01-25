use crate::core::adb_command::AdbArgs;
use crate::core::adb_device::AdbDevice;
use crate::core::ext::OutputExt;
use crate::core::r#const::{OFF, ON, SHELL};
use crate::core::selector::{resolve_device, run_adb_with};
use std::process::ExitCode;

const GET_TOUCHES: &str = "settings get system show_touches";
const PUT_TOUCHES: &str = "settings put system show_touches";

pub fn is_taps_on(device: &AdbDevice) -> bool {
    run_adb_with(device, AdbArgs::run(&[SHELL, GET_TOUCHES])).stdout() == ON
}

pub fn turn_taps(device: &AdbDevice, on: bool) -> ExitCode {
    let value = match on {
        true => ON,
        false => OFF,
    };
    return run_adb_with(&device, AdbArgs::run(&[SHELL, PUT_TOUCHES, value]))
        .exit_code()
}

pub fn toggle_taps() -> ExitCode {
    let device = match resolve_device() {
        Ok(device) => device,
        Err(code) => return code,
    };
    return turn_taps(&device, !is_taps_on(&device))
}