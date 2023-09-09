use crate::core::adb_device::AdbDevice;
use crate::core::ext::{OutputExt, print_no_one};
use crate::core::strings::{NO_ADB, SELECT_DEVICE};
use crate::core::util::read_usize_or_in;
use std::env;
use std::process::{exit, Command, Output};
use crate::core::adb_command::AdbArgs;

const WHICH: &str = "/usr/bin/which";
const ADB: &str = "adb";
const ARG_DEVICES: &str = "devices";
const DEVICE: &str = "device";
const ARG_S: &str = "-s";

pub fn resolve_device_and_run_args() {
    let args = AdbArgs::spawn(get_args().as_slice());
    let mut output = run_adb(args.clone());
    if output.is_more_than_one() {
        let device = resolve_device();
        output = run_adb_with_device(&device, args);
    }
    output.print();
    exit(output.code());
}

pub fn run_adb_with_device(device: &AdbDevice, mut args: AdbArgs) -> Output {
    args.args.insert(0, device.name.clone());
    args.args.insert(0, ARG_S.to_string());
    return run_adb(args);
}

pub fn resolve_device() -> AdbDevice {
    let output = run_adb(AdbArgs::run(&[ARG_DEVICES]));
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
            print_no_one();
            exit(1);
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
        .filter(|(i, _)| *i >= 2)
        .map(|(_, it)| it)
        .collect::<Vec<String>>();
}

// fn spawn_adb<S: AsRef<OsStr>>(args: &[S]) -> Output {
//
// }

fn run_adb(args: AdbArgs) -> Output {
    let output = Command::new(WHICH)
        .arg(ADB)
        .output()
        .unwrap();
    let adb_path = output.stdout();
    if adb_path.is_empty() {
        NO_ADB.print();
        exit(1);
    }
    let mut adb = &mut Command::new(adb_path);
    for arg in args.args {
        adb = adb.arg(arg);
    }
    return if args.interactive {
        adb.spawn().unwrap()
            .wait_with_output().unwrap()
    } else {
        adb.output().unwrap()
    }
}
