use std::os::unix::prelude::ExitStatusExt;
use crate::core::adb_command::AdbArgs;
use crate::core::adb_device::{AdbDevice, AdbDeviceVec};
use crate::core::ext::{print_no_one, OutputExt, StringExt, VecExt};
use crate::core::fix::sudo_fix_on_linux;
use crate::core::r#const::{ERROR_CODE, SHELL};
use crate::core::strings::{CANCEL, ERROR, SELECT_DEVICE, UNAUTHORIZED_BY_DEVICE, UNKNOWN};
use crate::core::util::{failure, string};
use dialoguer::FuzzySelect;
use itertools::Itertools;
use std::process::{ExitCode, ExitStatus, Output};

const ARG_DEVICES: &str = "devices";
const DEVICE: &str = "device";
const UNAUTHORIZED: &str = "unauthorized";
const NO_PERMISSIONS: &str = "no permissions";
const ARG_S: &str = "-s";
const GETPROPS: &str = "
getprop ro.build.version.sdk;

getprop ro.product.brand;
getprop ro.product.manufacturer;
getprop ro.product.product.brand;
getprop ro.product.product.manufacturer;
getprop ro.product.system.brand;
getprop ro.product.system.manufacturer;
getprop ro.product.vendor.brand;
getprop ro.product.vendor.manufacturer;

print anime

getprop persist.sys.nt.device.name;
getprop ro.product.brand_device_name;

getprop ro.build.product;

getprop ro.product.model;
getprop ro.product.product.model;
getprop ro.product.system.model;
getprop ro.product.vendor.model;

getprop ro.product.device;
getprop ro.product.name;
getprop ro.product.odm.device;
getprop ro.product.odm.name;
getprop ro.product.vendor.device;
getprop ro.product.vendor.name;

getprop ro.product.product.device;
getprop ro.product.product.name;
getprop ro.product.system.name;
getprop ro.product.system_ext.device;
getprop ro.product.system_ext.name;
";

const VERSIONS: [&str; 37] = ["Astro Boy or Bender", "1.0", "1.1", "1.5", "1.6", "2.0 ", "2.0.1", "2.1", "2.2", "2.3.0–2", "2.3.3–7", "3.0", "3.1", "3.2", "4.0.1–2", "4.0.3–4", "4.1", "4.2", "4.3", "4.4", "4.4W", "5.0", "5.1", "6", "7.0", "7.1", "8.0", "8.1", "9", "10", "11", "12", "12L", "13", "14", "15", "16"];

const DEVICE_COMMANDS: [&str; 19] = [
    // file transfer
    "push",
    "pull",
    "sync",
    // shell
    "shell",
    //"emu",
    // app (un)installation
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

pub fn resolve_device_and_run_args(args: &[String]) -> ExitCode {
    let args = AdbArgs::spawn(args);
    let first = match args.args.first() {
        None => return run_adb(args).exit_code(),
        Some(first) => first,
    };
    let output = if DEVICE_COMMANDS.contains(&first.as_str()) {
        let device = match resolve_device() {
            Ok(device) => device,
            Err(code) => return code,
        };
        run_adb_with(&device, args)
    } else {
        run_adb(args)
    };
    return output.exit_code()
}

pub fn adb_args_with(device: &AdbDevice, mut args: AdbArgs) -> AdbArgs {
    args.args.insert(0, string(ARG_S));
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
            let model = if ok { get_description(&serial) } else { serial.clone() };
            AdbDevice { serial, model, ok, unauthorized, no_permissions }
        }).collect::<Vec<AdbDevice>>();
}

pub fn resolve_device() -> Result<AdbDevice, ExitCode> {
    let mut devices = fetch_adb_devices();
    let device = match () {
        _ if devices.is_empty() => {
            print_no_one();
            return failure();
        },
        _ if devices.len() == 1 => devices.remove(0),
        _ => ask_for_device(devices),
    };
    if device.no_permissions && !sudo_fix_on_linux(Some(device.serial.clone())) {
        ERROR.println();
        return failure();
    }
    return Ok(device);
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
        panic!("wtf");
    }
    return devices.remove(selection);
}

fn get_description(serial: &String) -> String {
    let output = run_adb(AdbArgs::run(&[ARG_S, serial.as_str(), SHELL, GETPROPS]));
    if !output.status.success() {
        return serial.clone();
    }
    let stdout = output.stdout();
    let mut properties = stdout.split('\n')
        .map(|it| string(it))
        .collect::<Vec<String>>();
    let sdk = properties.remove(0).parse::<usize>();
    let version = VERSIONS.get(sdk.clone().unwrap_or(VERSIONS.len())).unwrap_or(&"n/a");
    let version = format!("{version} [{}]", sdk.unwrap_or(0));

    let index = match properties.index_of(&string("anime")) {
        None => return serial.clone(),
        Some(index) => index,
    };
    let mut vendor = properties[0..index].iter()
        .find_or_first(|it| !it.is_empty())
        .map(|it| it.clone());
    let models = &properties[(index + 1)..properties.len()];

    let mut suitable: Vec<String> = vec![];
    for property in models {
        let mut skip = false;
        let prop = property.to_lowercase();
        for (i, it) in suitable.iter().enumerate() {
            let it = it.to_lowercase();
            match () {
                _ if prop == it => (),
                // this value is less complete
                _ if it.contains(prop.as_str()) => (),
                // this value is more complete
                _ if prop.contains(it.as_str()) => suitable[i] = property.clone(),
                // this value is unique for now
                _ => continue,
            }
            skip = true;
            break
        }
        if !skip {
            suitable.push(property.clone());
        }
    }
    if let Some(vendor_name) = vendor.clone() {
        suitable.sort_by(|first, second| {
            let first = first.contains_ci(&vendor_name);
            let second = second.contains_ci(&vendor_name);
            if first || second {
                vendor = None;
            }
            second.cmp(&first)
        })
    }
    let prefix = match vendor {
        Some(vendor) if suitable.is_empty() => vendor,
        Some(vendor) => format!("{vendor}: "),
        None => string(""),
    };
    return format!("{prefix}{}, serial: {serial}, Android {version}", suitable.join(", "))
}

fn run_adb(args: AdbArgs) -> Output {
    let interactive = args.interactive;
    let mut command = match args.command() {
        Ok(v) => v,
        Err(e) => return Output {
            status: ExitStatus::from_raw(ERROR_CODE),
            stdout: vec![],
            stderr: e.to_string().into_bytes(),
        }
    };
    if interactive {
        Output {
            status: command.spawn().unwrap().wait().unwrap(),
            stdout: vec![],
            stderr: vec![],
        }
    } else {
        command.output().unwrap()
    }
}
