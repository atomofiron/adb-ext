use crate::core::ext::Rslt;
use crate::core::utils::adb_args::AdbArgs;
use crate::core::utils::r#const::SHELL;
use crate::core::utils::selector::run_adb;
use crate::core::utils::strings::SELECT_ANIM_SCALE;
use crate::core::utils::{interactive_select, string};

const WINDOW: &str = "settings put global window_animation_scale";
const TRANSITION: &str = "settings put global transition_animation_scale";
const ANIMATOR: &str = "settings put global animator_duration_scale";

const SCALES: &[&str] = &["0", "0.5", "1", "1.5", "2", "5", "10"];

pub fn change_anim_scale(scale: String) -> Rslt<()> {
    let scale = match scale.as_str() {
        s if SCALES.contains(&s) => s,
        _ => interactive_select(SELECT_ANIM_SCALE.value(), SCALES.to_vec(), |it, _| string(it))?,
    };
    let cmd = format!("{WINDOW} {scale}; {TRANSITION} {scale}; {ANIMATOR} {scale};");
    let args = &[SHELL, cmd.as_str()];
    let output = run_adb(AdbArgs::run(args))?;
    return output.to_rslt()
}
