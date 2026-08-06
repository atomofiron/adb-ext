use crate::core::ext::Rslt;
use crate::core::ext::result::ResultExt;
use crate::core::utils::adb_args::AdbArgs;
use crate::core::utils::adb_device::AdbDevice;
use crate::core::utils::r#const::{OFF, ON, SHELL};
use crate::core::utils::selector::{resolve_device, run_adb_for};

const GET_TOUCHES: &str = "settings get system show_touches";
const PUT_TOUCHES: &str = "settings put system show_touches";

pub fn is_taps_on(device: &AdbDevice) -> Rslt<bool> {
    run_adb_for(AdbArgs::run(&[SHELL, GET_TOUCHES]), device.serial.clone())
        .map(|mut output| output.stdout() == ON)
}

pub fn turn_taps(device: &AdbDevice, on: bool) -> Rslt<()> {
    let value = match on {
        true => ON,
        false => OFF,
    };
    return run_adb_for(AdbArgs::run(&[SHELL, PUT_TOUCHES, value]), device.serial.clone()).unit()
}

pub fn toggle_taps() -> Rslt<()> {
    let device = resolve_device()?;
    let now = is_taps_on(&device)?;
    return turn_taps(&device, !now)
}