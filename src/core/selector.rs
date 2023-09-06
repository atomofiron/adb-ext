use std::ffi::OsStr;
use std::env;
use std::process::{Command, exit, Output};
use crate::core::ext::OutputString;
use crate::core::strings::{NO_ADB, SELECT_DEVICE};
use crate::core::util::{exit_err, read_usize_or_in};

const WHICH: &str = "/usr/bin/which";
const ADB: &str = "adb";
const DEVICES: &str = "devices";
const DEVICE: &str = "device";
const MORE_THAN_ONE: &str = "adb: more than one device/emulator";

struct Device {
    pub name: String,
    pub authorized: bool,
}

// todo add support stdin or idk
pub fn run_with_device() {
    let output = run_with(None);
    if is_more_than_one(&output) {
        ask_for_device_and_run();
    } else {
        print_and_exit(output);
    }
}

fn is_more_than_one(output: &Output) -> bool {
    !output.status.success() && output.stderr() == MORE_THAN_ONE
}

fn print_and_exit(output: Output) {
    let stdout = output.stdout();
    if !stdout.is_empty() {
        println!("{stdout}");
    }
    let stderr = output.stderr();
    if !stderr.is_empty() {
        println!("{stderr}");
    }
    exit(output.status.code().unwrap_or(1));
}

pub fn ask_for_device_and_run() {
    let output = run_adb(&[DEVICES]);
    let devices = output.stdout().split('\n')
        .enumerate()
        .filter_map(|(i,it)|
            // the first is "List of devices attached"
            if i == 0 { None } else {
                let parts = it.split("\t").collect::<Vec<&str>>();
                let device = Device {
                    name: String::from(parts[0]),
                    authorized: parts[1] == DEVICE,
                };
                Some(device)
            }
        ).collect::<Vec<Device>>();
    let device = ask_for_device(devices);
    let output = run_with(Some(device));
    print_and_exit(output);
}

fn ask_for_device(mut devices: Vec<Device>) -> Device {
    for (i, device) in devices.iter().enumerate() {
        let status = if device.authorized { "" } else { " (unauthorized)" };
        println!("{}) {}{status}", i + 1, device.name)
    }
    let index= read_usize_or_in(SELECT_DEVICE.value(), 1, 1..=devices.len()) - 1;
    return devices.remove(index);
}

fn run_with(device: Option<Device>) -> Output {
    let mut args = match device {
        None => vec![],
        Some(device) => vec!["-s".to_string(), device.name],
    };
    let input_args = &mut env::args()
        .enumerate()
        // ignore "*/green-pain" and "adb"
        .filter_map(|(i,it)| if i <= 1 { None } else { Some(it) })
        .collect::<Vec<String>>();
    args.append(input_args);
    return run_adb(&args);
}

fn run_adb<S: AsRef<OsStr>>(args: &[S]) -> Output {
    let output = Command::new(WHICH)
        .arg(ADB)
        .output()
        .unwrap();
    let adb = output.stdout();
    if adb.is_empty() {
        exit_err(NO_ADB.value());
        exit(1);
    }
    let mut adb = &mut Command::new(adb);
    for arg in args {
        adb = adb.arg(arg);
    }
    return adb.output().unwrap();
}
