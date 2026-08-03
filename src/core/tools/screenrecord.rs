use crate::core::adb_command::AdbArgs;
use crate::core::config::Config;
use crate::core::destination::Destination;
use crate::core::ext::path_buf::PathBufExt;
use crate::core::ext::print::PrintExt;
use crate::core::r#const::{PULL, SHELL};
use crate::core::selector::{resolve_device, run_adb_for};
use crate::core::strings::{PRESS_ENTER_TO_STOP_REC, SAVED};
use crate::core::system::interrupt;
use crate::core::tools::taps::{is_taps_on, turn_taps};
use crate::core::util::{ensure_parent_exists, format_file_name, try_run_hook_and_exit};
use std::io;
use std::process::ExitCode;
use std::thread::sleep;
use std::time::Duration;

const SCREENRECORD: &str = "screenrecord";
const TMP: &str = "/data/local/tmp/record.mp4";

pub fn make_screencast(cmd: String, dst: String, config: &Config) -> ExitCode {
    let device = match resolve_device() {
        Ok(device) => device,
        Err(code) => return code,
    };
    let show_taps = config.screencasts.show_taps;
    let toggle_taps = show_taps != is_taps_on(&device);
    if toggle_taps {
        turn_taps(&device, show_taps);
    }
    let args = &[SHELL, SCREENRECORD, &config.screencasts.args, TMP];
    let mut command = match AdbArgs::spawn(args).to_command(Some(device.serial.clone())) {
        Ok(c) => c,
        Err(e) => {
            e.eprintln();
            return ExitCode::FAILURE;
        }
    };
    #[cfg(windows)] {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x0000_0200u32); // CREATE_NEW_PROCESS_GROUP
    }
    let mut child = command.spawn().unwrap();
    PRESS_ENTER_TO_STOP_REC.print();
    io::stdin().read_line(&mut String::new()).unwrap();
    interrupt(child.id());
    child.wait().unwrap();
    if toggle_taps {
        turn_taps(&device, !show_taps);
    }
    sleep(Duration::from_secs(1));
    let dst = dst
        .dst_with_parent(&config.screencasts.destination)
        .join(format_file_name(&config.screencasts.name));
    ensure_parent_exists(&dst);
    let mut output = run_adb_for(AdbArgs::run(&[PULL, TMP, dst.to_str()]), device.serial);
    output.print_out_and_err();
    let mut code = output.code;
    if output.success() {
        SAVED.println_formatted(&[&dst.to_string()]);
        code = config.screencast_hook()
            .map(|hook| try_run_hook_and_exit(hook, cmd, dst))
            .unwrap_or(code)
    }
    return code
}
