mod core;

use std::fmt::{format};
use std::fs::{File, OpenOptions};
use std::{fs, io};
use std::io::{Read, Seek, Write, Error};
use std::ops::RangeInclusive;
use std::process::{Command, exit, Stdio};
use std::thread::sleep;
use std::time::Duration;
use crate::core::device::Device;
use crate::core::strings::*;
use crate::core::util::*;

static LSUSB: &str = "lsusb";
static TARGET_DIR: &str = "/etc/udev/rules.d/";
static TARGET_FILE: &str = "/etc/udev/rules.d/51-android.rules";
static NEW_LINE: &str = "\n";
// SUBSYSTEM=="usb", ATTR{idVendor}=="04e8", MODE="0666", GROUP="plugdev", SYMLINK+="android%n"
// SUBSYSTEMS=="usb", ATTRS{idVendor}=="12d1", ATTRS{idProduct} =="1038", MODE="0666", OWNER="<username>"
static VENDOR_ID_PLACE_HOLDER: &str = "vendor_id";
static PAYLOAD: &str = "SUBSYSTEM==\"usb\", ATTR{idVendor}==\"vendor_id\", MODE=\"0666\", GROUP=\"plugdev\", SYMLINK+=\"android%n\"";

fn main() {
    unsafe {
        let id = geteuid();
        println!("id {}", id);
    }

    Language::set_language(Language::Ru);

    if true {
        let d = Device {
            vendor_id: String::from("1234"),
            product_id: String::from("5678"),
            description: String::from("qwerty"),
        };
        apply(d);
        return;
    }

    let mut devices = find_devices();

    if devices.is_empty() {
        NO_DEVICES_FOUND.println();
        exit(0);
    }
    let mut index = 0;
    if devices.len() > 1 {
        index = ask_target_device(&devices);
    }
    let device = devices.remove(index);

    match apply(device) {
        true => SUCCESSFULLY.println(),
        false => ERROR.println(),
    }
}

fn find_devices() -> Vec<Device> {
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
    return diffs.iter()
        .map(|it| Device::from(it))
        .collect::<Vec<Device>>();
}

fn fetch_lsusb() -> Result<Vec<String>, String> {
    let output = Command::new(LSUSB).output().unwrap();
    if !output.status.success() {
        let error = String::from_utf8(output.stderr).unwrap();
        return Err(error)
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

fn ask_target_device(devices: &Vec<Device>) -> usize {
    TYPE_TARGET_INDEX.println();
    for i in 0..devices.len() {
        println!("{}: {}", i + 1, devices[i].description);
    }
    return read_usize_in(TARGET_INDEX.value(), 1..=devices.len()) - 1;
}

fn apply(device: Device) -> bool {
    if let Err(why) = add_to_config(device) {
        println!("{}", why);
        return false;
    }

    let mut success = Command::new("udevadm").arg("control").arg("--reload-rules").status().unwrap().success();
    success = success && Command::new("udevadm").arg("trigger").arg("--reload-rules").status().unwrap().success();
    success = success || Command::new("service").arg("udev").arg("restart").status().unwrap().success();

    return success;
}

fn add_to_config(device: Device) -> Result<(),Error> {
    fs::create_dir_all(TARGET_DIR)?;
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(TARGET_FILE)?;

    let payload = PAYLOAD.replace(VENDOR_ID_PLACE_HOLDER, device.vendor_id.as_str());
    file.write_all(NEW_LINE.as_bytes())?;
    file.write_all(payload.as_bytes())?;
    file.write_all(NEW_LINE.as_bytes())?;
    return Ok(());
}
