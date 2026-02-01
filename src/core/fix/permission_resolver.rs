use crate::core::ext::{OptionArg, OutputExt, PrintExt};
use crate::core::fix::usb_device::UsbDevice;
use crate::core::selector::fetch_adb_devices;
use crate::core::strings::{NO_DEVICES_FOUND, RECONNECT_DEVICES, SUDO_EXPLANATION, UNKNOWN_ERROR, WELL_DONE};
use crate::FIX;
use itertools::Itertools;
use nix::unistd::Uid;
use rusb::UsbContext;
use std::fs;
use std::io::{Error, ErrorKind, Write};
use std::path::Path;
use std::process::{Command, ExitCode};
use std::thread::sleep;
use std::time::Duration;

const SUDO: &str = "sudo";
const TARGET_FILE: &str = "/etc/udev/rules.d/51-android.rules";
// SUBSYSTEM=="usb", ATTR{idVendor}=="04e8", MODE="0666", GROUP="plugdev", SYMLINK+="android%n"
// SUBSYSTEMS=="usb", ATTRS{idVendor}=="12d1", ATTRS{idProduct} =="1038", MODE="0666", OWNER="<username>"
const VENDOR_ID_PLACE_HOLDER: &str = "vendor_id";
const PAYLOAD: &str = "\nSUBSYSTEM==\"usb\", ATTR{idVendor}==\"vendor_id\", MODE=\"0666\", GROUP=\"plugdev\", SYMLINK+=\"android%n\"";


pub fn sudo_fix_permission(serial: Option<String>) -> ExitCode {
    SUDO_EXPLANATION.println();
    let path = std::env::current_exe().unwrap();
    return Command::new(SUDO)
        .arg(path)
        .arg(FIX)
        .some_arg(serial)
        .output()
        .map(|output| output.exit_code())
        .unwrap_or(ExitCode::FAILURE);
}

pub fn fix_permission(serial: Option<String>) -> ExitCode {
    if !Uid::current().is_root() {
        return sudo_fix_permission(serial)
    }
    let serials = fetch_adb_devices()
        .into_iter()
        .filter_map(|it| if it.no_permissions { Some(it.serial) } else { None })
        .collect::<Vec<String>>();
    let ids = find_usb_devices(serial.clone())
        .into_iter()
        .filter_map(|it| if serials.contains(&it.serial) { Some(it.vendor_id) } else { None })
        .unique()
        .collect::<Vec<String>>();
    if ids.is_empty() {
        NO_DEVICES_FOUND.println();
        return ExitCode::FAILURE;
    }
    return match apply(&ids) {
        Err(cause) => {
            cause.println();
            ExitCode::FAILURE
        },
        _ => match serial {
            None => {
                RECONNECT_DEVICES.println();
                ExitCode::SUCCESS
            },
            Some(serial) => {
                RECONNECT_DEVICES.println();
                wait_for_the_fixed_adb_device(serial);
                WELL_DONE.println();
                ExitCode::SUCCESS
            },
        }
    }
}

fn find_usb_devices(serial: Option<String>) -> Vec<UsbDevice> {
    let mut devices = vec![];
    let context = rusb::Context::new().unwrap();
    for device in context.devices().unwrap().iter() {
        let handle = match device.open() {
            Ok(value) => value,
            Err(_) => continue, // NoDevice: No such device (it may have been disconnected)
        };
        let timeout = Duration::from_secs(1);
        let languages = handle.read_languages(timeout).unwrap();
        let language = languages.first().unwrap().clone();
        let device_des = if let Ok(des) = device.device_descriptor() { des } else { continue };
        let config = device.active_config_descriptor().map(|it| {
            handle.read_configuration_string(language, &it, timeout).unwrap_or(String::new())
        }).unwrap_or(String::new());
        let number = handle.read_serial_number_string(language, &device_des, timeout)
            .unwrap_or(String::new());
        let device = UsbDevice {
            vendor_id: format!("{:04x}", device_des.vendor_id()),
            product_id: format!("{:04x}", device_des.product_id()),
            serial: number.clone(),
        };
        match &serial {
            Some(serial) if number == *serial => return vec![device],
            None if config == "adb" => devices.push(device),
            _ => (),
        }
    }
    return devices;
}

fn apply(ids: &Vec<String>) -> Result<(), Error> {
    add_to_config(ids)?;
    restart_service()?;
    return Ok(());
}

fn restart_service() -> Result<(), Error> {
    let mut success = Command::new("udevadm")
        .arg("control")
        .arg("--reload-rules")
        .status()?
        .success();
    success = success && Command::new("udevadm").arg("trigger").status()?.success();
    success = success
        || Command::new("service")
            .arg("udev")
            .arg("restart")
            .status()?
            .success();
    match success {
        true => Ok(()),
        false => Err(Error::new(ErrorKind::Other, UNKNOWN_ERROR.value())),
    }
}

fn add_to_config(ids: &Vec<String>) -> Result<(), Error> {
    let path = Path::new(TARGET_FILE);
    fs::create_dir_all(path.parent().unwrap())?;
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(path)?;

    for device in ids {
        let line = PAYLOAD.replace(VENDOR_ID_PLACE_HOLDER, device);
        file.write_all(line.as_bytes())?;
    }
    return Ok(());
}

fn wait_for_the_fixed_adb_device(serial: String) {
    while fetch_adb_devices().into_iter()
        .find(|it| it.serial == serial && !it.no_permissions)
        .is_none() {
        sleep(Duration::from_secs(1));
    }
}
