use crate::core::adb_command::AdbArgs;
use crate::core::adb_device::AdbDevice;
use crate::core::ext::OutputExt;
use crate::core::r#const::{OFF, ON, SHELL};
use crate::core::selector::{resolve_device, run_adb_with};

const GET_TOUCHES: &str = "settings get system show_touches";
const PUT_TOUCHES: &str = "settings put system show_touches";

pub fn is_taps_on(device: &AdbDevice) -> bool {
    run_adb_with(device, AdbArgs::run(&[SHELL, GET_TOUCHES])).stdout() == ON
}

pub fn turn_taps(device: &AdbDevice, on: bool) {
    let value = match on {
        true => ON,
        false => OFF,
    };
    run_adb_with(&device, AdbArgs::run(&[SHELL, PUT_TOUCHES, value]));
}

pub fn toggle_taps() {
    let device = resolve_device();
    turn_taps(&device, !is_taps_on(&device))
}