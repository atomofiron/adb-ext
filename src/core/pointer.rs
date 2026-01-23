use crate::core::adb_command::AdbArgs;
use crate::core::adb_device::AdbDevice;
use crate::core::ext::OutputExt;
use crate::core::r#const::{OFF, ON, SHELL};
use crate::core::selector::{resolve_device, run_adb_with};

const GET_TOUCHES: &str = "settings get system pointer_location";
const PUT_TOUCHES: &str = "settings put system pointer_location";

pub fn is_pointer_on(device: &AdbDevice) -> bool {
    run_adb_with(device, AdbArgs::run(&[SHELL, GET_TOUCHES])).stdout() == ON
}

pub fn turn_pointer(device: &AdbDevice, on: bool) {
    let value = match on {
        true => ON,
        false => OFF,
    };
    run_adb_with(&device, AdbArgs::run(&[SHELL, PUT_TOUCHES, value]));
}

pub fn toggle_pointer() {
    let device = resolve_device();
    turn_pointer(&device, !is_pointer_on(&device))
}