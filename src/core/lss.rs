use crate::core::adb_command::AdbArgs;
use crate::core::ext::OutputExt;
use crate::core::selector::{resolve_device, run_adb_with_device};
use crate::core::strings::SCREENSHOTS_NOT_FOUND;
use crate::core::util::SHELL;
use std::process::exit;

const PICTURES_SCREENSHOTS: &str = "/sdcard/Pictures/Screenshots/";
const DCIM_SCREENSHOTS: &str = "/sdcard/DCIM/Screenshots/";
const DESKTOP_SCREENSHOTS: &str = "/Pictures/Screenshots/";
const EXISTS: &str = "exists";

pub fn pull_screenshots(count: usize) {
    let device = resolve_device();
    let mut path = DCIM_SCREENSHOTS;
    let mut check_args = AdbArgs::run(&[SHELL, "test", "-d", path, "&&", "echo", EXISTS]);
    let mut stdout = run_adb_with_device(&device, check_args.clone()).stdout();
    if stdout != EXISTS {
        path = PICTURES_SCREENSHOTS;
        check_args.args[3] = PICTURES_SCREENSHOTS.to_string();
        stdout = run_adb_with_device(&device, check_args).stdout();
    }
    if stdout != EXISTS {
        SCREENSHOTS_NOT_FOUND.print();
        exit(1);
    }
    let ls_args = AdbArgs::run(&[SHELL, "ls", "-t", path]);
    let output = run_adb_with_device(&device, ls_args);
    if output.status.success() {
        let mut pull_args = AdbArgs::run(&["pull"]);
        let stdout = output.stdout();
        let lines = stdout.lines().take(count);
        for line in lines {
            pull_args.args.push(format!("{path}{line}"));
        }
        #[allow(deprecated)] // todo replace with a crate
        let mut dst = std::env::home_dir().unwrap().to_str().unwrap().to_string();
        dst = format!("{dst}{DESKTOP_SCREENSHOTS}");
        std::fs::create_dir_all(dst.clone()).unwrap();
        pull_args.args.push(dst);
        let output = run_adb_with_device(&device, pull_args);
        output.print();
        exit(output.code());
    } else {
        println!("{}", output.stderr());
        exit(output.code());
    };
}
