use crate::core::adb_command::AdbArgs;
use crate::core::config::Config;
use crate::core::destination::Destination;
use crate::core::ext::{OutputExt, PathBufExt, VecExt};
use crate::core::r#const::SHELL;
use crate::core::selector::{resolve_device, run_adb_with};
use crate::core::strings::SAVED;
use crate::core::util::{ensure_parent_exists, format_file_name, try_run_hook_and_exit};
use std::fs;
use std::process::exit;

const SCREENCAP_P: &str = "screencap -p";
const OD: u8 = 0x0D;
const OA: u8 = 0x0A;

pub fn make_screenshot(cmd: String, dst: String, config: &Config) {
    let device = resolve_device();
    let args = &[SHELL, SCREENCAP_P];
    let output = run_adb_with(&device, AdbArgs::run(args));

    if output.status.success() {
        let dst = dst
            .dst_with_parent(&config.screenshots.destination)
            .join(format_file_name(&config.screenshots.name));
        ensure_parent_exists(&dst);

        let bytes = match &output.stdout[4..=5] {
            &[OD, OA] => output.stdout,
            _ => filter_extra_zero_d(output.stdout),
        };
        fs::write(&dst, bytes).unwrap();
        println!("{SAVED}: {}", dst.to_string());
        let hook = config.screenshot_hook();
        try_run_hook_and_exit(hook, cmd, dst);
    } else {
        output.print_err();
        exit(output.code());
    }
}

fn filter_extra_zero_d(src: Vec<u8>) -> Vec<u8> {
    let mut dst = Vec::new();
    for i in 0..src.len() {
        let byte = src[i];
        if byte != OD || i == src.last_index() || src[i + 1] != OA {
            dst.push(byte)
        }
    }
    return dst;
}
