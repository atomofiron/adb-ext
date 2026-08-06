use crate::core::ext::Rslt;
use crate::core::ext::path_buf::PathBufExt;
use crate::core::ext::print::PrintExt;
use crate::core::util::adb_args::AdbArgs;
use crate::core::util::config::Config;
use crate::core::util::destination::Destination;
use crate::core::util::r#const::{PULL, SHELL};
use crate::core::tools::taps::{is_taps_on, turn_taps};
use crate::core::util::selector::{resolve_device, run_adb_for};
use crate::core::util::strings::{PRESS_ENTER_TO_STOP_REC, SAVED};
use crate::core::util::system::interrupt;
use crate::core::util::{ensure_parent_exists, format_file_name, try_run_hook_and_exit};
use std::io;
use std::thread::sleep;
use std::time::Duration;

const SCREENRECORD: &str = "screenrecord";
const TMP: &str = "/data/local/tmp/record.mp4";

pub fn make_screencast(cmd: String, dst: String, config: &Config) -> Rslt<()> {
    let device = resolve_device()?;
    let show_taps = config.screencasts.show_taps;
    let toggle_taps = show_taps != is_taps_on(&device)?;
    if toggle_taps {
        turn_taps(&device, show_taps)?;
    }
    let args = &[SHELL, SCREENRECORD, &config.screencasts.args, TMP];
    let mut command = AdbArgs::spawn(args).to_command(Some(device.serial.clone()))?;
    #[cfg(windows)] {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x0000_0200u32); // CREATE_NEW_PROCESS_GROUP
    }
    let mut child = command.spawn()?;
    PRESS_ENTER_TO_STOP_REC.print();
    io::stdin().read_line(&mut String::new())?;
    interrupt(child.id());
    child.wait()?;
    if toggle_taps {
        turn_taps(&device, !show_taps)?;
    }
    sleep(Duration::from_secs(1));
    let dst = dst
        .dst_with_parent(&config.screencasts.destination)
        .join(format_file_name(&config.screencasts.name));
    ensure_parent_exists(&dst)?;
    let mut output = run_adb_for(AdbArgs::run(&[PULL, TMP, dst.to_str()]), device.serial)?;
    output.print_out_and_err();
    if output.success() {
        SAVED.println_formatted(&[&dst.to_string()]);
        let hook = config.screencast_hook();
        if let Some(rslt) = hook.map(|hook| try_run_hook_and_exit(hook, cmd, dst)) {
            return rslt;
        };
    }
    return output.to_rslt()
}
