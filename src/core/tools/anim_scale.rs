use crate::core::adb_command::AdbArgs;
use crate::core::r#const::SHELL;
use crate::core::selector::run_adb;
use crate::core::strings::SELECT_ANIM_SCALE;
use crate::core::util::{interactive_select, string};
use std::process::ExitCode;

const WINDOW: &str = "settings put global window_animation_scale";
const TRANSITION: &str = "settings put global transition_animation_scale";
const ANIMATOR: &str = "settings put global animator_duration_scale";

const SCALES: &[&str] = &["0", "0.5", "1", "1.5", "2", "5", "10"];

pub fn change_anim_scale(scale: String) -> ExitCode {
    let scale = match scale.as_str() {
        s if SCALES.contains(&s) => s,
        _ => match interactive_select(SELECT_ANIM_SCALE.value(), SCALES.to_vec(), |it, _| string(it)) {
            Ok(s) => s,
            Err(code) => return code,
        },
    };
    let cmd = format!("{WINDOW} {scale}; {TRANSITION} {scale}; {ANIMATOR} {scale};");
    let args = &[SHELL, cmd.as_str()];
    let mut output = run_adb(AdbArgs::run(args));
    if !output.success() {
        output.print_err();
    }
    return output.code
}
