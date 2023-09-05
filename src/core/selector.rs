use std::io;
use std::io::Write;
use std::process::{Command, exit, Output};
use crate::core::strings::NO_ADB;
use crate::core::util::exit_err;

const WHICH: &str = "/usr/bin/which";
const ADB: &str = "adb";
const DEVICES: &str = "devices";
const DEVICE: &str = "device";

struct Device {
    pub name: String,
    pub authorized: bool,
}

pub fn run_with_device() {
    let output = Command::new(WHICH)
        .arg(ADB)
        .output()
        .unwrap();
    let output = trimmed(output);
    if output.is_empty() {
        exit_err(NO_ADB.value());
        exit(1);
    }
    let output = Command::new(output)
        .arg(DEVICES)
        .output()
        .unwrap();
    let output = trimmed(output);
    let devices = output.split('\n')
        .enumerate()
        .filter_map(|(i,it)|
            // the first is "List of devices attached"
            if i == 0 { None } else {
                let parts = it.split("\t").collect::<Vec<&str>>();
                let device = Device {
                    name: String::from(parts[0]),
                    authorized: parts[1] == DEVICE,
                };
                Some(device)
            }
        ).collect::<Vec<Device>>();
    if devices.len() < 2 {
        run_with(None);
    } else {
        run_with(Some(ask_for_device(devices)));
    }
}

fn run_with(device: Option<Device>) {

}

fn ask_for_device(mut devices: Vec<Device>) -> Device {
    for (i, device) in devices.iter().enumerate() {
        let status = if device.authorized { "" } else { " (unauthorized)" };
        println!("{}) {}{status}", i + 1, device.name)
    }
    let mut value: Option<usize> = None;
    while value.is_none() || !(1..=devices.len()).contains(&value.unwrap()) {
        print!("pick the device (default 1): ");
        io::stdout().flush().unwrap();
        value = read_int(Some(1));
    }
    return devices.remove(value.unwrap() - 1);
}

fn read_int(default: Option<usize>) -> Option<usize> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    if input.is_empty() && default.is_some() {
        return default;
    }
    return input
        .parse::<usize>()
        .map_or(None, |it| Some(it));
}

fn trimmed(output: Output) -> String {
    String::from(String::from_utf8(output.stdout).unwrap().trim())
}