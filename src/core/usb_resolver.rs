use crate::core::ext::OptionArg;
use crate::core::strings::*;
use crate::core::usb_device::UsbDevice;
use nix::unistd::Uid;
use std::io::{Error, ErrorKind, Write};
use std::process::{exit, Command};
use std::time::Duration;
use std::fs;
use dialoguer::FuzzySelect;
use crate::ARG_FIX;
use crate::core::adb_device::AdbDevice;
use crate::core::selector::fetch_devices;

const SUDO: &str = "sudo";
const TARGET_DIR: &str = "/etc/udev/rules.d/";
const TARGET_FILE: &str = "/etc/udev/rules.d/51-android.rules";
// SUBSYSTEM=="usb", ATTR{idVendor}=="04e8", MODE="0666", GROUP="plugdev", SYMLINK+="android%n"
// SUBSYSTEMS=="usb", ATTRS{idVendor}=="12d1", ATTRS{idProduct} =="1038", MODE="0666", OWNER="<username>"
const VENDOR_ID_PLACE_HOLDER: &str = "vendor_id";
const PAYLOAD: &str = "SUBSYSTEM==\"usb\", ATTR{idVendor}==\"vendor_id\", MODE=\"0666\", GROUP=\"plugdev\", SYMLINK+=\"android%n\"";


fn find_device(serial: String) -> Option<UsbDevice> {
    let context = libusb::Context::new().unwrap();
    for device in context.devices().unwrap().iter() {
        let descriptor = device.device_descriptor().unwrap();
        let handle = device.open().unwrap();
        let timeout = Duration::from_secs(1);
        let languages = handle.read_languages(timeout).unwrap();
        let language = languages.first().unwrap().clone();
        let number = handle.read_serial_number_string(language, &descriptor, timeout)
            .unwrap_or(String::new());
        let manufacturer = handle.read_manufacturer_string(language, &descriptor, timeout)
            .unwrap_or(String::new());
        let product = handle.read_product_string(language, &descriptor, timeout)
            .unwrap_or(String::new());
        /* adb
        let config = device.active_config_descriptor().unwrap();
        let config = handle.read_configuration_string(language, &config, timeout)
            .unwrap_or(String::new());*/
        if number == serial {
            let vendor_id = format!("{:04x}", descriptor.vendor_id());
            let product_id = format!("{:04x}", descriptor.product_id());
            let device = UsbDevice { vendor_id, product_id, description: format!("{manufacturer} {product}, {number}") };
            return Some(device);
        }
    }
    return None;
}

pub fn resolve_permission(serial: Option<String>) {
    if !Uid::current().is_root() {
        exit(fix_with_sudo(serial));
    }
    let devices = fetch_devices();
    let serial = match () {
        _ if serial.is_some() => serial.unwrap(),
        _ if devices.is_empty() => return NO_DEVICES_FOUND.println(),
        _ if devices.len() == 1 => match devices.first() {
            Some(a) => a.serial.clone(),
            None => return DEVICE_NOT_FOUND.println(),
        },
        _ => serial.unwrap_or_else(|| ask_device(&devices)),
    };
    let device = match find_device(serial.clone()) {
        Some(device) => device,
        None => return DEVICE_NOT_FOUND.println(),
    };
    match apply(&device) {
        Ok(_) => SUCCESSFULLY.println(),
        Err(cause) => println!("{}", cause),
    }
}

fn ask_device(devices: &Vec<AdbDevice>) -> String {
    let mut items = devices.iter()
        .map(|it| it.serial.clone())
        .collect::<Vec<String>>();
    items.push(CANCEL.value().to_string());
    let selection = FuzzySelect::new()
        .with_prompt(SELECT_DEVICE.value())
        .default(0)
        .items(&items)
        .interact()
        .unwrap();
    devices.get(selection)
        .unwrap_or_else(|| exit(0))
        .serial.clone()
}

fn apply(device: &UsbDevice) -> Result<(), Error> {
    add_to_config(device)?;
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

fn add_to_config(device: &UsbDevice) -> Result<(), Error> {
    fs::create_dir_all(TARGET_DIR)?;
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(TARGET_FILE)?;

    let payload = PAYLOAD.replace(VENDOR_ID_PLACE_HOLDER, device.vendor_id.as_str());
    file.write_all('\n'.to_string().as_bytes())?;
    file.write_all(payload.as_bytes())?;
    return Ok(());
}

pub fn fix_with_sudo(serial: Option<String>) -> i32 {
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
