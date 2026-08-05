use crate::core::adb_args::AdbArgs;
use crate::core::config::Config;
use crate::core::destination::Destination;
use crate::core::ext::path_buf::PathBufExt;
use crate::core::ext::vec::VecExt;
use crate::core::ext::Rslt;
use crate::core::r#const::SHELL;
use crate::core::selector::run_adb;
use crate::core::strings::SAVED;
use crate::core::util::{ensure_parent_exists, format_file_name, try_run_hook_and_exit};
use std::fs;

const SCREENCAP_P: &str = "screencap -p";
const OD: u8 = 0x0D;
const OA: u8 = 0x0A;

pub fn make_screenshot(cmd: String, dst: String, config: &Config) -> Rslt<()> {
    let args = &[SHELL, SCREENCAP_P];
    let output = run_adb(AdbArgs::run(args))?;

    if output.success() {
        let dst = dst
            .dst_with_parent(&config.screenshots.destination)
            .join(format_file_name(&config.screenshots.name));
        ensure_parent_exists(&dst)?;

        let bytes = match &output.stdout[4..=5] {
            &[OD, OA] => &output.stdout,
            _ => &filter_extra_zero_d(&output.stdout),
        };
        fs::write(&dst, bytes)?;
        SAVED.println_formatted(&[&dst.to_string()]);
        let hook = config.screenshot_hook();
        if let Some(rslt) = hook.map(|hook| try_run_hook_and_exit(hook, cmd, dst)) {
            return rslt
        }
    }
    return output.to_rslt()
}

fn filter_extra_zero_d(src: &Vec<u8>) -> Vec<u8> {
    let mut dst = Vec::new();
    for i in 0..src.len() {
        let byte = src[i];
        if byte != OD || i == src.last_index() || src[i + 1] != OA {
            dst.push(byte)
        }
    }
    return dst;
}
