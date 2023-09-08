use crate::core::adb_device::AdbDevice;
use crate::core::ext::OutputExt;
use crate::core::strings::{NO_ADB, SELECT_DEVICE};
use crate::core::util::{exit_err, read_usize_or_in, SHELL};
use std::env;
use std::ffi::OsStr;
use std::process::{exit, Command, Output};

const WHICH: &str = "/usr/bin/which";
const ADB: &str = "adb";
const ARG_DEVICES: &str = "devices";
const DEVICE: &str = "device";
const ARG_S: &str = "-s";

pub fn resolve_device_and_run_args() {
    let args = get_args();
    let mut output = run_adb(args.as_slice());
    if output.is_more_than_one() {
        let device = resolve_device();
        output = run_adb_with_device(&device, args);
    }
    output.print();
    exit(output.code());
}

pub fn run_adb_with_device(device: &AdbDevice, mut args: Vec<String>) -> Output {
    args.insert(0, device.name.clone());
    args.insert(0, ARG_S.to_string());
    return run_adb(args.as_slice());
}

pub fn resolve_device() -> AdbDevice {
    let output = run_adb(&[ARG_DEVICES]);
    let mut devices = output.stdout().split('\n')
        .enumerate()
        .filter_map(|(i,it)|
            // the first line is "List of devices attached"
            if i == 0 { None } else {
                let parts = it.split("\t").collect::<Vec<&str>>();
                let device = AdbDevice {
                    name: parts[0].to_string(),
                    authorized: parts[1] == DEVICE,
                };
                Some(device)
            }
        ).collect::<Vec<AdbDevice>>();
    return match () {
        _ if devices.is_empty() => {
            let output = run_adb(&[SHELL]);
            output.print();
            exit(output.code());
        },
        _ if devices.len() == 1 => devices.remove(0),
        _ => ask_for_device(devices),
    };
}

fn ask_for_device(mut devices: Vec<AdbDevice>) -> AdbDevice {
    for (i, device) in devices.iter().enumerate() {
        let status = if device.authorized { "" } else { " (unauthorized)" };
        println!("{}) {}{status}", i + 1, device.name)
    }
    let index = read_usize_or_in(SELECT_DEVICE.value(), 1, 1..=devices.len()) - 1;
    return devices.remove(index);
}

fn get_args() -> Vec<String> {
    return env::args()
        .enumerate()
        // ignore "*/green-pain" and "adb"
        .filter_map(|(i, it)| if i <= 1 { None } else { Some(it) })
        .collect::<Vec<String>>();
}

// todo add support stdin or idk
fn run_adb<S: AsRef<OsStr>>(args: &[S]) -> Output {
    let output = Command::new(WHICH).arg(ADB).output().unwrap();
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
