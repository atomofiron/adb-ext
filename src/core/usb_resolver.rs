use crate::core::ext::Split;
use crate::core::strings::*;
use crate::core::usb_device::UsbDevice;
use nix::unistd::Uid;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Write};
use std::process::{exit, Command};
use std::thread::sleep;
use std::time::Duration;
use std::{fs, io};
use dialoguer::FuzzySelect;

const SUDO: &str = "sudo";
const LSUSB: &str = "lsusb";
const TARGET_DIR: &str = "/etc/udev/rules.d/";
const TARGET_FILE: &str = "/etc/udev/rules.d/51-android.rules";
// SUBSYSTEM=="usb", ATTR{idVendor}=="04e8", MODE="0666", GROUP="plugdev", SYMLINK+="android%n"
// SUBSYSTEMS=="usb", ATTRS{idVendor}=="12d1", ATTRS{idProduct} =="1038", MODE="0666", OWNER="<username>"
const VENDOR_ID_PLACE_HOLDER: &str = "vendor_id";
const PAYLOAD: &str = "SUBSYSTEM==\"usb\", ATTR{idVendor}==\"vendor_id\", MODE=\"0666\", GROUP=\"plugdev\", SYMLINK+=\"android%n\"";

pub fn resolve_permission() {
    if !Uid::current().is_root() {
        exit(rerun_with_sudo());
    }

    let devices = find_devices();

    if devices.is_empty() {
        NO_DEVICES_FOUND.println();
        exit(0);
    }
    let mut index = 0;
    if devices.len() > 1 {
        index = ask_target_device_or_exit(&devices);
    }
    let device = devices.get(index).unwrap();

    match apply(device) {
        true => SUCCESSFULLY.println(),
        false => ERROR.println(),
    }
}

fn find_devices() -> Vec<UsbDevice> {
    let lines_before = fetch_lsusb().unwrap();

    CONNECT_OR_DISCONNECT.print();

    io::stdin().read_line(&mut String::new()).unwrap();

    let lines_after = fetch_lsusb().unwrap();
    let mut diffs = find_diffs(&lines_before, &lines_after);

    if diffs.is_empty() {
        PLEASE_WAIT.println();
        sleep(Duration::from_secs(3));
        let lines_after = fetch_lsusb().unwrap();
        diffs = find_diffs(&lines_before, &lines_after);
    }
    return diffs
        .iter()
        .filter(|it| it.starts_with("Bus "))
        .map(UsbDevice::from)
        .collect::<Vec<UsbDevice>>();
}

fn fetch_lsusb() -> Result<Vec<String>, String> {
    let output = Command::new(LSUSB).output().unwrap();
    if !output.status.success() {
        let error = String::from_utf8(output.stderr).unwrap();
        return Err(error);
    }
    let result = String::from_utf8(output.stdout)
        .unwrap()
        .trim()
        .split_to_vec('\n');
    return Ok(result);
}

fn find_diffs(first: &Vec<String>, second: &Vec<String>) -> Vec<String> {
    let mut diffs = Vec::new();
    for line in first {
        if !second.contains(line) {
            diffs.push(line.clone());
        }
    }
    for line in second {
        if !first.contains(line) {
            diffs.push(line.clone());
        }
    }
    diffs
}

fn ask_target_device_or_exit(devices: &Vec<UsbDevice>) -> usize {
    let items = devices.iter().map(|it| it.description.clone()).collect::<Vec<String>>();
    let selection = FuzzySelect::new()
        .with_prompt(SELECT_DEVICE.value())
        .default(0)
        .items(&items)
        .interact()
        .unwrap();
    if selection >= devices.len() {
        exit(0);
    }
    return selection;
}

fn apply(device: &UsbDevice) -> bool {
    if let Err(cause) = add_to_config(device) {
        println!("{}", cause);
        return false;
    }
    if let Err(cause) = restart_service() {
        println!("{}", cause);
        return false;
    }
    return true;
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
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(TARGET_FILE)?;

    let payload = PAYLOAD.replace(VENDOR_ID_PLACE_HOLDER, device.vendor_id.as_str());
    file.write_all('\n'.to_string().as_bytes())?;
    file.write_all(payload.as_bytes())?;
    return Ok(());
}

fn rerun_with_sudo() -> i32 {
    let path = std::env::current_exe().unwrap();
    return Command::new(SUDO)
        .arg(path)
        .status()
        .unwrap()
        .code()
        .unwrap_or(1);
}
