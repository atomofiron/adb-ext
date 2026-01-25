use crate::core::adb_command::AdbArgs;
use crate::core::ext::OutputExt;
use crate::core::r#const::SHELL;
use crate::core::selector::{resolve_device, run_adb_with};
use std::process::ExitCode;

const GET_PROP: &str = "getprop debug.layout"; // getprop debug.layout
const SET_PROP: &str = "setprop debug.layout"; // getprop debug.layout
const GET_SETTING: &str = "settings get global debug_layout"; // settings get global debug_layout
const PUT_SETTING: &str = "settings put global debug_layout"; // settings get global debug_layout
const CALL: &str = "service call activity 1599295570";

pub fn debug_layout_bounds() -> ExitCode {
    let invert_prop = invert(GET_PROP);
    let invert_setting = invert(GET_SETTING);
    let command = format!("{SET_PROP} $({invert_prop}); {PUT_SETTING} $({invert_setting}); {CALL}");
    let args = &[SHELL, command.as_str()];
    let device = match resolve_device() {
        Ok(device) => device,
        Err(code) => return code,
    };
    let output = run_adb_with(&device, AdbArgs::run(args));

    if !output.status.success() {
        output.print_err();
    }
    return output.exit_code()
}

fn invert(get_cmd: &str) -> String {
    format!("v=\"$({get_cmd})\"; case \"$v\" in true) echo false;; false) echo true;; esac")
}
