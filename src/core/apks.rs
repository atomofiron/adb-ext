use std::fs;
use std::ops::Add;
use std::path::Path;
use std::process::{Command, exit, Output};
use regex::Regex;
use crate::core::adb_command::AdbArgs;
use crate::core::adb_device::AdbDevice;
use crate::core::config::{Config, CONFIG_PATH};
use crate::core::destination::Destination;
use crate::core::ext::{OutputExt, StrExt};
use crate::core::r#const::{INSTALL, PULL, SHELL};
use crate::core::selector::{resolve_device, run_adb_with};
use crate::core::strings::{NO_BUILD_TOOLS, NO_FILE};

pub fn steal_apk(package: String, dst: Option<String>) {
    let pm_command = format!("pm path {package}");
    let args = AdbArgs::run(&[SHELL, pm_command.as_str()]);
    let device = resolve_device();
    let output = run_adb_with(&device, args);
    if !output.status.success() {
        output.print_err();
        exit(output.code());
    }
    let default_name = format!("{package}.apk");
    let destination = dst
        .unwrap_or(default_name.clone())
        .with_file(default_name.as_str());
    // the output line is "package:/data/data/[…]/base.apk"
    let path = &output.stdout().clone()[8..];
    let args = AdbArgs::spawn(&[PULL, path, destination.as_str()]);
    let output = run_adb_with(&device, args);
    exit(output.code());
}

pub fn run_apk(apk: String, config: &Config) {
    if !Path::new(&apk).exists() {
        NO_FILE.exit_err();
    }
    let aapt = get_aapt(&config);
    let device = resolve_device();
    let output = install(&device, &apk);
    if !output.status.success() {
        exit(output.code());
    }
    let (package, activity) = get_package_activity(aapt, &apk);
    let output = launch(&device, package, activity);
    exit(output.code());
}

fn get_aapt(config: &Config) -> String {
    let path = match config.build_tools() {
        None => {
            println!("{NO_BUILD_TOOLS} {}", CONFIG_PATH);
            exit(0);
        },
        Some(path) => path,
    };
    let pattern = Regex::new(r"/\d+\.\d\.\d$").unwrap();
    return fs::read_dir(&path).unwrap()
        .map(|it| it.unwrap().path().display().to_string())
        .filter(|it| pattern.is_match(it))
        .collect::<Vec<String>>()
        .first()
        .unwrap_or_else(|| {
            println!("{NO_BUILD_TOOLS} {}", CONFIG_PATH);
            exit(0);
        }).clone().add("/aapt");
}

fn install(device: &AdbDevice, apk: &String) -> Output {
    let args = AdbArgs::spawn(&[INSTALL, apk.as_str()]);
    return run_adb_with(&device, args);
}

fn get_package_activity(aapt: String, apk: &String) -> (String, String) {
    let text = Command::new(aapt)
        .arg("d").arg("xmltree").arg(apk).arg("AndroidManifest.xml")
        .output().unwrap()
        .stdout()
        .replace('\n', " ");
    let package = Regex::new(r#" A: package="[^"]+"#).unwrap();
    let package = package.find(&text).unwrap().as_str();
    let offset = package.index_of('"').unwrap() + 1;
    let package = package[offset..].to_string();
    let activity = Regex::new(r#" E: activity.+="android\.intent\.action\.MAIN""#).unwrap();
    let activity = activity.find(&text).unwrap().as_str();
    let name = Regex::new(r#" A: android:name\(0x\d{8}\)="[^"]+"#).unwrap();
    let activity = name.find(activity).unwrap().as_str();
    let offset = activity.index_of('"').unwrap() + 1;
    let activity = activity[offset..].to_string();
    return (package, activity);
}

fn launch(device: &AdbDevice, package: String, activity: String) -> Output {
    let command = format!("am start -a android.intent.action.MAIN -n {package}/{activity}");
    let args = AdbArgs::spawn(&[SHELL, command.as_str()]);
    return run_adb_with(&device, args);
}
