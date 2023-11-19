extern crate chrono;

use std::fs;
use crate::core::adb_command::AdbArgs;
use crate::core::ext::{OutputExt, VecExt};
use crate::core::preparing::{Destination, DestinationExt};
use crate::core::selector::{resolve_device, run_adb_with};
use crate::core::r#const::{DESKTOP_SCREENSHOTS, SHELL};
use std::process::exit;
use crate::core::strings::SAVED;
use crate::core::util::ensure_parent_exists;

const SCREENCAP: &str = "screencap -p";
const NAME_TEMPLATE: &str = "Screenshot_%Y-%m-%d_%H-%M-%S.png";
const OD: u8 = 0x0D;
const OA: u8 = 0x0A;

pub fn make_screenshot(dst: Option<String>) {
    let device = resolve_device();
    let args = &[SHELL, SCREENCAP];
    let output = run_adb_with(&device, AdbArgs::run(args));

    if output.status.success() {
        let dst = Destination::from(dst, DESKTOP_SCREENSHOTS)
            .replace_tilde()
            .with_file(NAME_TEMPLATE);
        ensure_parent_exists(&dst);

        let bytes = match &output.stdout[4..=5] {
            &[OD, OA] => output.stdout,
            _ => filter_extra_zero_d(output.stdout),
        };
        fs::write(dst.clone(), bytes).unwrap();
        println!("{}: {dst}", SAVED.value());
    } else {
        output.print_err();
        exit(output.code());
    }
}

fn filter_extra_zero_d(src: Vec<u8>) -> Vec<u8> {
    let mut dst = Vec::new();
    for i in 0..(src.last_index() - 1) {
        let byte = src[i];
        if byte != OD || src[i + 1] != OA {
            dst.push(byte)
        }
    }
    return dst;
}
