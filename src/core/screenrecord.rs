use crate::core::adb_command::AdbArgs;
use crate::core::ext::OutputExt;
use crate::core::destination::Destination;
use crate::core::selector::{resolve_device, run_adb_with};
use crate::core::r#const::{PULL, SHELL};
use std::process::exit;
use crate::core::config::get_config;
use crate::core::strings::DESTINATION;
use crate::core::util::ensure_parent_exists;

const SCREENRECORD: &str = "screenrecord";

pub fn make_screencast(dst: String) {
    let config = get_config();
    let tmp = "/data/local/tmp/record.mp4";
    let args = &[SHELL, SCREENRECORD, &config.screencasts.args, &tmp];
    let device = resolve_device();
    let output = run_adb_with(&device, AdbArgs::spawn(args));

    if output.status.success() {
        let dst = dst
            .with_dir(&config.screenshots.destination)
            .with_file(&config.screenshots.name);
        ensure_parent_exists(&dst);
        let output = run_adb_with(&device, AdbArgs::run(&[PULL, tmp, &dst]));
        output.print_out_and_err();
        if output.status.success() {
            DESTINATION.print();
            println!("{dst}");
        }
        exit(output.code());
    } else {
        output.print_err();
        exit(output.code());
    }
}
