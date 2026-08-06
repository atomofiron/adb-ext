#![cfg(target_os = "linux")]

use crate::core::ext::command::CommandExt;
use crate::core::ext::output::OutputExt;
use crate::core::ext::print::PrintExt;
use crate::core::ext::Rslt;
use crate::core::fix::usb_device::UsbDevice;
use crate::core::r#const::ADB;
use crate::core::selector::fetch_adb_devices;
use crate::core::strings::{NO_DEVICES_FOUND, NO_PARENT, RECONNECT_DEVICES, SUDO_EXPLANATION, UNKNOWN_ERROR, WELL_DONE};
use crate::FIX;
use itertools::Itertools;
use nix::unistd::Uid;
use rusb::UsbContext;
use std::fs;
use std::io::{Error, ErrorKind, Write};
use std::path::Path;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

const SUDO: &str = "sudo";
const TARGET_FILE: &str = "/etc/udev/rules.d/51-android.rules";
// SUBSYSTEM=="usb", ATTR{idVendor}=="04e8", MODE="0666", GROUP="plugdev", SYMLINK+="android%n"
// SUBSYSTEMS=="usb", ATTRS{idVendor}=="12d1", ATTRS{idProduct} =="1038", MODE="0666", OWNER="<username>"
const VENDOR_ID_PLACE_HOLDER: &str = "vendor_id";
const PAYLOAD: &str = "\nSUBSYSTEM==\"usb\", ATTR{idVendor}==\"vendor_id\", MODE=\"0666\", GROUP=\"plugdev\", SYMLINK+=\"android%n\"";


pub fn sudo_fix_permission(serial: Option<String>) -> Rslt<()> {
    SUDO_EXPLANATION.println();
    let path = std::env::current_exe()?;
    return Command::new(SUDO)
        .arg(path)
        .arg(FIX)
        .some_arg(serial)
        .output()?
        .convert()
        .to_rslt();
}

pub fn fix_permission(serial: Option<String>) -> Rslt<()> {
    if !Uid::current().is_root() {
        return sudo_fix_permission(serial)
    }
    let serials = fetch_adb_devices()?
        .into_iter()
        .filter_map(|it| if it.no_permissions { Some(it.serial) } else { None })
        .collect::<Vec<String>>();
    let ids = find_usb_devices(serial.clone())?
        .into_iter()
        .filter_map(|it| if serials.contains(&it.serial) { Some(it.vendor_id) } else { None })
        .unique()
        .collect::<Vec<String>>();
    if ids.is_empty() {
        return NO_DEVICES_FOUND.to_err();
    }
    apply(&ids)?;
    match serial {
        None => RECONNECT_DEVICES.println(),
        Some(serial) => {
            RECONNECT_DEVICES.println();
            wait_for_the_fixed_adb_device(serial)?;
            WELL_DONE.println();
        },
    }
    return Ok(())
}

fn find_usb_devices(serial: Option<String>) -> Rslt<Vec<UsbDevice>> {
    let mut devices = vec![];
    let context = rusb::Context::new()?;
    for device in context.devices()?.iter() {
        let handle = match device.open() {
            Ok(value) => value,
            Err(_) => continue, // NoDevice: No such device (it may have been disconnected)
        };
        let timeout = Duration::from_secs(1);
        let languages = handle.read_languages(timeout)?;
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
            Some(serial) if number == *serial => return Ok(vec![device]),
            None if config == ADB => devices.push(device),
            _ => (),
        }
    }
    return Ok(devices);
}

fn apply(ids: &Vec<String>) -> Rslt<()> {
    add_to_config(ids)?;
    restart_service()?;
    return Ok(());
}

fn restart_service() -> Rslt<()> {
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
        false => Err(Error::new(ErrorKind::Other, UNKNOWN_ERROR.value()).into()),
    }
}

fn add_to_config(ids: &Vec<String>) -> Rslt<()> {
    let path = Path::new(TARGET_FILE);
    let parent = path.parent()
        .ok_or_else(|| NO_PARENT.value())?;
    fs::create_dir_all(parent)?;
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

fn wait_for_the_fixed_adb_device(serial: String) -> Rslt<()> {
    while fetch_adb_devices()?
        .into_iter()
        .find(|it| it.serial == serial && !it.no_permissions)
        .is_none() {
        sleep(Duration::from_secs(1));
    }
    Ok(())
}
