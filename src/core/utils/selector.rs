use crate::core::ext::Rslt;
use crate::core::ext::exit_status::ExitStatusExt;
use crate::core::ext::output::OutputExt;
use crate::core::ext::print::PrintExt;
use crate::core::ext::string::StringExt;
use crate::core::ext::vec::VecExt;
use crate::core::fix::sudo_fix_on_linux;
use crate::core::utils::adb_args::AdbArgs;
use crate::core::utils::adb_device::{AdbDevice, AdbDeviceVec};
use crate::core::utils::output::Output;
use crate::core::utils::values::SHELL;
use crate::core::utils::strings::{ADB_NO_DEVICES_FOUND, SELECT_DEVICE, UNAUTHORIZED_BY_DEVICE, UNKNOWN};
use crate::core::utils::{interactive_select, string};
use itertools::Itertools;
use std::io;
use std::io::{BufRead, BufReader};
use std::process::{Child, Stdio};

const MANY_TARGETS: &str = "more than one device/emulator";

const ARG_DEVICES: &str = "devices";
const DEVICE: &str = "device";
const UNAUTHORIZED: &str = "unauthorized";
const NO_PERMISSIONS: &str = "no permissions";
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

const VERSIONS: [&str; 38] = [
    "Astro Boy or Bender", "1.0", "1.1", "1.5", "1.6", "2.0 ", "2.0.1", "2.1", "2.2", "2.3.0–2", "2.3.3–7", "3.0",
    "3.1", "3.2", "4.0.1–2", "4.0.3–4", "4.1", "4.2", "4.3", "4.4", "4.4W", "5.0", "5.1", "6", "7.0", "7.1", "8.0",
    "8.1", "9", "10", "11", "12", "12L", "13", "14", "15", "16", "17"
];

pub fn fetch_adb_devices() -> Rslt<Vec<AdbDevice>> {
    let mut output = run_adb(AdbArgs::run(&[ARG_DEVICES]))?;
    let devices = output.stdout().split('\n')
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
    return Ok(devices)
}

pub fn resolve_device() -> Rslt<AdbDevice> {
    let mut devices = fetch_adb_devices()?;
    let device = match () {
        _ if devices.is_empty() => {
            return ADB_NO_DEVICES_FOUND.to_err()
        },
        _ if devices.len() == 1 => devices.remove(0),
        _ => ask_for_device(devices)?,
    };
    if device.no_permissions {
        sudo_fix_on_linux(Some(device.serial.clone()))?;
    }
    return Ok(device);
}

fn ask_for_device(devices: Vec<AdbDevice>) -> Rslt<AdbDevice> {
    interactive_select(SELECT_DEVICE.value(), devices, |device, devices| {
        let status = match () {
            _ if device.ok => String::new(),
            _ if device.unauthorized => format!(" ({UNAUTHORIZED_BY_DEVICE})").to_lowercase(),
            _ => format!(" ({UNKNOWN})").to_lowercase(),
        };
        format!("{}{status}", devices.get_unique_model_name(device))
    })
}

fn get_description(serial: &String) -> String {
    let mut output = match run_adb_for(AdbArgs::run(&[SHELL, GETPROPS]), serial.clone()) {
        Err(_) => return serial.clone(),
        Ok(output) => output,
    };
    let stdout = output.stdout();
    let mut properties = stdout.split('\n')
        .map(|it| string(it))
        .collect::<Vec<String>>();
    let sdk = properties.remove(0).parse::<usize>();
    let version = VERSIONS.get(sdk.clone().unwrap_or(VERSIONS.len())).unwrap_or(&"n/a");
    let version = format!("{version} [{}]", sdk.unwrap_or(0));

    let index = match properties.index_of(|it| it == "anime") {
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

pub fn run_adb(args: AdbArgs) -> Rslt<Output> {
    run(args, None)
}

pub fn run_adb_for(args: AdbArgs, device: String) -> Rslt<Output> {
    run(args, Some(device))
}

fn run(args: AdbArgs, device: Option<String>) -> Rslt<Output> {
    let device_specified = device.is_some();
    let mut command = args.clone().to_command(device)?;
    return if args.interactive {
        if !device_specified {
            command.stderr(Stdio::piped());
        }
        let mut child = command.spawn()?;
        if !device_specified {
            if let Some(output) = resolve_and_restart_if_many_devices(&mut child, args)? {
                return Ok(output)
            }
        }
        Ok(Output::from(child.wait()?.exit_code()))
    } else {
        let output =command.output()?.convert();
        let index = output.stderr.index_of(|a| *a == b'\n')
            .unwrap_or(output.stderr.len());
        let first = String::from_utf8_lossy(&output.stderr[0..index]);
        if first[0..index].ends_with(MANY_TARGETS) {
            return resolve_device_and_run(args)
        }
        Ok(output)
    }
}

fn resolve_and_restart_if_many_devices(child: &mut Child, args: AdbArgs) -> Rslt<Option<Output>> {
    if let Some(stderr_pipe) = child.stderr.take() {
        let mut reader = BufReader::new(stderr_pipe);
        let mut line = String::new();
        if reader.read_line(&mut line).unwrap_or(0) > 0 {
            match line.trim() {
                str if str.ends_with(MANY_TARGETS) => {
                    return Ok(Some(resolve_device_and_run(args)?))
                },
                _ => line.println(),
            }
        }
        io::copy(&mut reader, &mut io::stderr())?;
    }
    return Ok(None)
}

fn resolve_device_and_run(args: AdbArgs) -> Rslt<Output> {
    let device = resolve_device()?;
    return run_adb_for(args, device.serial)
}
