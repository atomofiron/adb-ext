use crate::core::adb_command::AdbArgs;
use crate::core::ext::OutputExt;
use crate::core::destination::Destination;
use crate::core::selector::{adb_args_with, resolve_device, run_adb_with};
use crate::core::r#const::{PULL, SHELL};
use std::process::exit;
use std::io;
use std::thread::sleep;
use std::time::Duration;
use nix::libc::pid_t;
use nix::sys::signal;
use nix::sys::signal::Signal;
use nix::unistd::Pid;
use crate::core::config::Config;
use crate::core::strings::{DESTINATION, PRESS_ENTER_TO_STOP_REC};
use crate::core::util::{ensure_parent_exists, try_run_hook_and_exit};

const SCREENRECORD: &str = "screenrecord";

pub fn make_screencast(cmd: String, dst: String, config: &Config) {
    let tmp = "/data/local/tmp/record.mp4";
    let args = &[SHELL, SCREENRECORD, &config.screencasts.args, &tmp];
    let device = resolve_device();
    let args = adb_args_with(&device, AdbArgs::spawn(args));
    let mut command = args.command();
    let mut child = command.spawn().unwrap();
    PRESS_ENTER_TO_STOP_REC.print();
    io::stdin().read_line(&mut String::new()).unwrap();
    signal::kill(Pid::from_raw(child.id() as pid_t), Signal::SIGINT).unwrap();
    child.wait().unwrap();
    sleep(Duration::from_secs(1));
    let dst = dst
        .dst_with_parent(&config.screencasts.destination)
        .with_file(&config.screencasts.name);
    ensure_parent_exists(&dst);
    let output = run_adb_with(&device, AdbArgs::run(&[PULL, tmp, &dst]));
    output.print_out_and_err();
    if output.status.success() {
        DESTINATION.print();
        println!("{dst}");
        try_run_hook_and_exit(config.screencast_hook(), cmd, dst)
    }
    exit(output.code());
}
