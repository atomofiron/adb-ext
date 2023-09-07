use crate::core::ext::{OutputExt, StringVec, VecExt};
use crate::core::selector::{resolve_device, run_adb_with_device};
use crate::core::strings::SCREENSHOTS_NOT_FOUND;
use crate::core::util::SHELL;
use std::process::exit;

const PICTURES_SCREENSHOTS: &str = "/sdcard/Pictures/Screenshots/";
const DCIM_SCREENSHOTS: &str = "/sdcard/DCIM/Screenshots/";

pub fn pull_screenshots(count: usize) {
    // adb shell ls -1td /sdcard/Pictures/Screenshots/*
    // adb shell ls -1td /sdcard/DCIM/Screenshots/*
    // adb pull $files ~/Pictures/Screenshots/

    let device = resolve_device();

    let mut path = PICTURES_SCREENSHOTS;
    let mut check_args = vec![SHELL, "file", path].to_string_vec();
    let mut exists = run_adb_with_device(&device, check_args.clone())
        .status
        .success();
    if !exists {
        path = DCIM_SCREENSHOTS;
        let last_index = check_args.last_index();
        check_args[last_index] = String::from(path);
        exists = run_adb_with_device(&device, check_args.clone())
            .status
            .success();
    }
    if !exists {
        SCREENSHOTS_NOT_FOUND.print();
        exit(1);
    }
    let ls_args = vec![SHELL, "ls", "-t", path].to_string_vec();
    let output = run_adb_with_device(&device, ls_args);
    if output.status.success() {
        let mut pull_args = vec!["pull".to_string()];
        let stdout = output.stdout();
        let lines = stdout.lines().take(count);
        for line in lines {
            pull_args.push(format!("{path}{line}"));
        }
        #[allow(deprecated)] // todo replace with a crate
        let mut dst = std::env::home_dir().unwrap().to_str().unwrap().to_string();
        dst = format!("{dst}/Pictures/Screenshots/");
        std::fs::create_dir_all(dst.clone()).unwrap();
        pull_args.push(dst);
        let output = run_adb_with_device(&device, pull_args);
        exit(output.print_and_get_code());
    } else {
        println!("{}", output.stderr());
        exit(output.code());
    }
}
