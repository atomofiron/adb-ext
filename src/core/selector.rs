use crate::core::adb_device::{AdbDevice, AdbDeviceVec};
use crate::core::ext::{OutputExt, print_no_one, StrExt};
use crate::core::strings::{CANCEL, ERROR, SELECT_DEVICE, UNAUTHORIZED_BY_DEVICE, UNKNOWN};
use crate::core::r#const::{ERROR_CODE, SHELL, SUCCESS_CODE};
use std::env;
use std::process::{exit, Output};
use dialoguer::FuzzySelect;
use crate::core::adb_command::AdbArgs;
use crate::core::fix::sudo_fix_on_linux;

const ARG_DEVICES: &str = "devices";
const DEVICE: &str = "device";
const UNAUTHORIZED: &str = "unauthorized";
const NO_PERMISSIONS: &str = "no permissions";
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
    let args = AdbArgs::spawn(get_adb_args().as_slice());
    let first = args.args.first();
    let output = match first {
        None => run_adb(args),
        Some(first) if !DEVICE_COMMANDS.contains(&first.as_str()) => run_adb(args),
        _ => run_adb_with(&resolve_device(), args)
    };
    exit(output.code());
}

pub fn adb_args_with(device: &AdbDevice, mut args: AdbArgs) -> AdbArgs {
    args.args.insert(0, ARG_S.to_string());
    args.args.insert(1, device.serial.clone());
    return args;
}

pub fn run_adb_with(device: &AdbDevice, args: AdbArgs) -> Output {
    run_adb(adb_args_with(device, args))
}

pub fn fetch_adb_devices() -> Vec<AdbDevice> {
    let output = run_adb(AdbArgs::run(&[ARG_DEVICES]));
    return output.stdout().split('\n')
        .enumerate()
        // the first line is "List of devices attached"
        .filter(|(i, _)| *i > 0)
        .map(|(_, it)| {
            let parts = it.split('\t').collect::<Vec<&str>>();
            let serial = parts[0].to_string();
            let ok = parts[1] == DEVICE;
            let unauthorized = parts[1] == UNAUTHORIZED;
            let no_permissions = parts[1].starts_with(NO_PERMISSIONS);
            let model = if ok { get_model(&serial) } else { serial.clone() };
            AdbDevice { serial, model, ok, unauthorized, no_permissions }
        }).collect::<Vec<AdbDevice>>();
}

pub fn resolve_device() -> AdbDevice {
    let mut devices = fetch_adb_devices();
    let device = match () {
        _ if devices.is_empty() => {
            print_no_one();
            exit(ERROR_CODE);
        },
        _ if devices.len() == 1 => devices.remove(0),
        _ => ask_for_device(devices),
    };
    if device.no_permissions && sudo_fix_on_linux(Some(device.serial.clone())) != SUCCESS_CODE {
        ERROR.println();
        exit(ERROR_CODE);
    }
    return device;
}

fn ask_for_device(mut devices: Vec<AdbDevice>) -> AdbDevice {
    let mut items = devices.iter().map(|device| {
        let status = match () {
            _ if device.ok => String::new(),
            _ if device.unauthorized => format!(" ({UNAUTHORIZED_BY_DEVICE})").to_lowercase(),
            _ => format!(" ({UNKNOWN})").to_lowercase(),
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
        exit(SUCCESS_CODE);
    }
    return devices.remove(selection);
}

fn get_adb_args() -> Vec<String> {
    let mut args = env::args().collect::<Vec<String>>();
    // ignore "adb-ext" (or another command name or path)
    args.remove(0);
    return args;
}

fn get_model(name: &String) -> String {
    let output = run_adb(AdbArgs::run(&[ARG_S, name.as_str(), SHELL, GETPROPS])).stdout();
    let props = output.split('\n').collect::<Vec<&str>>();
    let mut suitable = "";
    for prop in props {
        suitable = match () {
            _ if prop.len() > suitable.len() => prop,
            _ if prop.contains(' ') && !suitable.contains(' ') => prop,
            _ if prop.contains_upper() && !suitable.contains_upper() => prop,
            _ => suitable,
        };
    }
    return if suitable.is_empty() { name.clone() } else { suitable.to_string() }
}

fn run_adb(args: AdbArgs) -> Output {
    if args.interactive {
        Output {
            status: args.command().spawn().unwrap().wait().unwrap(),
            stdout: vec![],
            stderr: vec![],
        }
    } else {
        args.command().output().unwrap()
    }
}
