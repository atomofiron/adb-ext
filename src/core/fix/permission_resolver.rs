use crate::core::ext::OptionArg;
use crate::core::strings::*;
use crate::core::usb_device::UsbDevice;
use nix::unistd::Uid;
use std::io::{Error, ErrorKind, Write};
use std::process::{exit, Command};
use std::fs;
use std::path::Path;
use std::time::Duration;
use itertools::Itertools;
use crate::ARG_FIX;
use crate::core::r#const::{ERROR_CODE, SUCCESS_CODE};

const SUDO: &str = "sudo";
const TARGET_FILE: &str = "/etc/udev/rules.d/51-android.rules";
// SUBSYSTEM=="usb", ATTR{idVendor}=="04e8", MODE="0666", GROUP="plugdev", SYMLINK+="android%n"
// SUBSYSTEMS=="usb", ATTRS{idVendor}=="12d1", ATTRS{idProduct} =="1038", MODE="0666", OWNER="<username>"
const VENDOR_ID_PLACE_HOLDER: &str = "vendor_id";
const PAYLOAD: &str = "\nSUBSYSTEM==\"usb\", ATTR{idVendor}==\"vendor_id\", MODE=\"0666\", GROUP=\"plugdev\", SYMLINK+=\"android%n\"";


pub fn sudo_fix_permission(serial: Option<String>) -> i32 {
    let path = std::env::current_exe().unwrap();
    return Command::new(SUDO)
        .arg(path)
        .arg(ARG_FIX)
        .some_arg(serial)
        .status()
        .unwrap()
        .code()
        .unwrap_or(1);
}

pub fn fix_permission(serial: Option<String>) {
    if !Uid::current().is_root() {
        exit(sudo_fix_permission(serial));
    }
    let ids = find_devices(serial.clone())
        .into_iter()
        .map(|it| it.vendor_id)
        .unique()
        .collect::<Vec<String>>();
    if ids.is_empty() {
        return NO_DEVICES_FOUND.println();
    };
    match apply(&ids) {
        Ok(_) => {
            SUCCESSFULLY.println();
            exit(SUCCESS_CODE);
        },
        Err(cause) => {
            println!("{}", cause);
            exit(ERROR_CODE);
        },
    }
}

fn find_devices(serial: Option<String>) -> Vec<UsbDevice> {
    let mut devices = vec![];
    let context = libusb::Context::new().unwrap();
    for device in context.devices().unwrap().iter() {
        let handle = device.open().unwrap();
        let timeout = Duration::from_secs(1);
        let languages = handle.read_languages(timeout).unwrap();
        let language = languages.first().unwrap().clone();
        let device_des = device.device_descriptor().unwrap();
        let config_des = device.active_config_descriptor().unwrap();
        let number = handle.read_serial_number_string(language, &device_des, timeout)
            .unwrap_or(String::new());
        let config = handle.read_configuration_string(language, &config_des, timeout)
            .unwrap_or(String::new());
        let device = UsbDevice {
            vendor_id: format!("{:04x}", device_des.vendor_id()),
            product_id: format!("{:04x}", device_des.product_id()),
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
