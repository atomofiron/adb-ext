use crate::core::adb_device::{AdbDevice, AdbDeviceVec};
use crate::core::ext::{OutputExt, print_no_one, StrExt};
use crate::core::strings::{CANCEL, NO_ADB, SELECT_DEVICE};
use crate::core::util::{NEW_LINE, read_usize_or_in, SHELL, SPACE, TAB};
use std::env;
use std::process::{exit, Command, Output};
use dialoguer::FuzzySelect;
use crate::core::adb_command::AdbArgs;

const WHICH: &str = "/usr/bin/which";
const ADB: &str = "adb";
const ARG_DEVICES: &str = "devices";
const DEVICE: &str = "device";
const UNAUTHORIZED: &str = "unauthorized";
const ARG_S: &str = "-s";
const GETPROPS: &str = "
getprop persist.sys.nt.device.name;
getprop ro.product.brand_device_name;

getprop ro.product.model
getprop ro.product.product.model
getprop ro.product.system.model
getprop ro.product.vendor.model

getprop ro.build.product;
getprop ro.product.bootimage.device;
getprop ro.product.bootimage.name;
getprop ro.product.device;
getprop ro.product.name;
getprop ro.product.odm.device;
getprop ro.product.odm.name;
getprop ro.product.vendor.device;
getprop ro.product.vendor.name;

getprop ro.product.product.device;
getprop ro.product.product.name;
getprop ro.product.vendor_dlkm.device;
getprop ro.product.vendor_dlkm.name;
getprop ro.product.system.name;
getprop ro.product.system_ext.device;
getprop ro.product.system_ext.name;
";
const DEVICE_COMMANDS: [&str; 19] = [
    // file transfer
    "push",
    "pull",
    "sync",
    // shell
    "shell",
    //"emu",
    // app installation
    "install",
    "install-multiple",
    "install-multi-package",
    "uninstall",
    // debugging
    "bugreport",
    "jdwp",
    "logcat",
    // scripting
    "get-state",
    "get-serialno",
    "get-devpath",
    "remount",
    "reboot",
    "sideload",
    // usb
    "attach",
    "detach",
];


pub fn resolve_device_and_run_args() {
    let args = AdbArgs::spawn(get_args().as_slice());
    let first = args.args.first();
    let output = match first {
        None => run_adb(args),
        _ if !DEVICE_COMMANDS.contains(&first.unwrap().as_str()) => run_adb(args),
        _ => run_adb_with(&resolve_device(), args)
    };
    exit(output.code());
}

pub fn run_adb_with(device: &AdbDevice, mut args: AdbArgs) -> Output {
    args.args.insert(0, ARG_S.to_string());
    args.args.insert(1, device.name.clone());
    return run_adb(args);
}

pub fn resolve_device() -> AdbDevice {
    let output = run_adb(AdbArgs::run(&[ARG_DEVICES]));
    let mut devices = output.stdout().split(NEW_LINE)
        .enumerate()
        // the first line is "List of devices attached"
        .filter(|(i, _)| *i > 0)
        .map(|(_, it)| {
            let parts = it.split(TAB).collect::<Vec<&str>>();
            let name = parts[0].to_string();
            let ok = parts[1] == DEVICE;
            let unauthorized = parts[1] == UNAUTHORIZED;
            let model = if ok { get_model(&name) } else { name.clone() };
            AdbDevice { name, model, ok, unauthorized }
        }).collect::<Vec<AdbDevice>>();
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
    let mut items = devices.iter().map(|device| {
        let status = match () {
            _ if device.ok => "",
            _ if device.unauthorized => " (unauthorized)",
            // todo => " (no permission)"
            _ => " (unknown)",
        };
        format!("{}{status}", devices.get_unique_model_name(device))
    }).collect::<Vec<String>>();
    items.push(CANCEL.value().to_string());
    let selection = FuzzySelect::new()
        .with_prompt(SELECT_DEVICE.value())
        .default(0)
        .items(&items)
        .interact()
        .unwrap();
    if selection >= devices.len() {
        exit(0);
    }
    return devices.remove(selection);
}

fn get_args() -> Vec<String> {
    return env::args()
        .enumerate()
        // ignore "*/green-pain" and "adb"
        .filter(|(i, _)| *i >= 2)
        .map(|(_, it)| it)
        .collect::<Vec<String>>();
}

fn get_model(name: &String) -> String {
    let output = run_adb(AdbArgs::run(&[ARG_S, name.as_str(), SHELL, GETPROPS])).stdout();
    let props = output.split(NEW_LINE).collect::<Vec<&str>>();
    let mut suitable = "";
    for prop in props {
        suitable = match () {
            _ if prop.len() > suitable.len() => prop,
            _ if prop.contains(SPACE) && !suitable.contains(SPACE) => prop,
            _ if prop.contains_upper() && !suitable.contains_upper() => prop,
            _ => suitable,
        };
    }
    return if suitable.is_empty() { name.clone() } else { suitable.to_string() }
}

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
