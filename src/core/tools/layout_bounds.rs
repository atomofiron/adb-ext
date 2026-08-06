use crate::core::ext::Rslt;
use crate::core::util::adb_args::AdbArgs;
use crate::core::util::r#const::SHELL;
use crate::core::util::selector::run_adb;

const GET_PROP: &str = "getprop debug.layout"; // getprop debug.layout
const SET_PROP: &str = "setprop debug.layout"; // getprop debug.layout
const GET_SETTING: &str = "settings get global debug_layout"; // settings get global debug_layout
const PUT_SETTING: &str = "settings put global debug_layout"; // settings get global debug_layout
const CALL: &str = "service call activity 1599295570";

pub fn debug_layout_bounds() -> Rslt<()> {
    let invert_prop = invert(GET_PROP);
    let invert_setting = invert(GET_SETTING);
    let command = format!("{SET_PROP} $({invert_prop}); {PUT_SETTING} $({invert_setting}); {CALL}");
    let args = &[SHELL, command.as_str()];
    let output = run_adb(AdbArgs::run(args))?;
    return output.to_rslt()
}

fn invert(get_cmd: &str) -> String {
    format!("v=\"$({get_cmd})\"; case \"$v\" in true) echo false;; false) echo true;; esac")
}
