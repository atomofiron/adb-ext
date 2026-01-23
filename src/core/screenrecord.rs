use crate::core::adb_command::AdbArgs;
use crate::core::config::Config;
use crate::core::destination::Destination;
use crate::core::ext::{OutputExt, PathBufExt};
use crate::core::r#const::{PULL, SHELL};
use crate::core::selector::{adb_args_with, resolve_device, run_adb_with};
use crate::core::strings::{DESTINATION, PRESS_ENTER_TO_STOP_REC};
use crate::core::system::kill;
use crate::core::util::{ensure_parent_exists, format_file_name, try_run_hook_and_exit};
use std::io;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use crate::core::taps::{is_taps_on, turn_taps};

const SCREENRECORD: &str = "screenrecord";
const TMP: &str = "/data/local/tmp/record.mp4";

pub fn make_screencast(cmd: String, dst: String, config: &Config) {
    let device = resolve_device();
    let show_taps = config.screencasts.show_taps;
    let toggle_taps = show_taps != is_taps_on(&device);
    if toggle_taps {
        turn_taps(&device, show_taps);
    }
    let args = &[SHELL, SCREENRECORD, &config.screencasts.args, TMP];
    let args = adb_args_with(&device, AdbArgs::spawn(args));
    let mut command = args.command();
    #[cfg(windows)] {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x0000_0200u32); // CREATE_NEW_PROCESS_GROUP
    }
    let mut child = command.spawn().unwrap();
    PRESS_ENTER_TO_STOP_REC.print();
    io::stdin().read_line(&mut String::new()).unwrap();
    kill(child.id());
    child.wait().unwrap();
    if toggle_taps {
        turn_taps(&device, !show_taps);
    }
    sleep(Duration::from_secs(1));
    let dst = dst
        .dst_with_parent(&config.screencasts.destination)
        .join(format_file_name(&config.screencasts.name));
    ensure_parent_exists(&dst);
    let output = run_adb_with(&device, AdbArgs::run(&[PULL, TMP, dst.to_str().unwrap()]));
    output.print_out_and_err();
    if output.status.success() {
        DESTINATION.print();
        println!("{}", dst.to_string());
        try_run_hook_and_exit(config.screencast_hook(), cmd, dst)
    }
    exit(output.code());
}
