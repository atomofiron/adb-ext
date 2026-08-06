use crate::core::ext::Rslt;
use crate::core::util::adb_args::AdbArgs;
use crate::core::util::adb_device::AdbDevice;
use crate::core::util::r#const::{OFF, ON, SHELL};
use crate::core::util::selector::{resolve_device, run_adb_for};

const GET_TOUCHES: &str = "settings get system pointer_location";
const PUT_TOUCHES: &str = "settings put system pointer_location";

pub fn is_pointer_on(device: &AdbDevice) -> Rslt<bool> {
    run_adb_for(AdbArgs::run(&[SHELL, GET_TOUCHES]), device.serial.clone())
        .map(|mut output| output.stdout() == ON)
}

pub fn turn_pointer(device: AdbDevice, on: bool) -> Rslt<()> {
    let value = match on {
        true => ON,
        false => OFF,
    };
    let output = run_adb_for(AdbArgs::run(&[SHELL, PUT_TOUCHES, value]), device.serial)?;
    return output.to_rslt()
}

pub fn toggle_pointer() -> Rslt<()> {
    let device = resolve_device()?;
    let now = is_pointer_on(&device)?;
    return turn_pointer(device, !now)
}