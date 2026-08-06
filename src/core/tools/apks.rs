use crate::core::adb_args::AdbArgs;
use crate::core::adb_device::AdbDevice;
use crate::core::config::Config;
use crate::core::destination::Destination;
use crate::core::ext::output::OutputExt;
use crate::core::ext::path_buf::PathBufExt;
use crate::core::ext::result::ResultExt;
use crate::core::ext::str::StrExt;
use crate::core::ext::Rslt;
use crate::core::output::Output;
use crate::core::r#const::{INSTALL, PULL, SHELL};
use crate::core::selector::{resolve_device, run_adb_for};
use crate::core::strings::{NO_ACTIVITY_FOUND, NO_ACTIVITY_NAME_FOUND, NO_ANDROID_SDK, NO_BUILD_TOOLS, NO_FILE, NO_PACKAGE_FOUND, NO_PACKAGE_NAME, NO_PATH, NO_QUOTE_FOUND, SAVED};
use crate::core::system::config_path;
use crate::core::util::string;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn steal_apk(package: Option<String>, dst: Option<String>) -> Rslt<()> {
    let package = match package {
        Some(package) => package,
        None => return NO_PACKAGE_NAME.to_err(),
    };
    let pm_command = format!("pm path {package}");
    let args = AdbArgs::run(&[SHELL, pm_command.as_str()]);
    let device = resolve_device()?;
    let mut output = run_adb_for(args, device.serial.clone())?;
    if !output.success() {
        return output.to_rslt()
    }
    let destination = dst
        .unwrap_or(string(""))
        .dst()
        .join(format!("{package}.apk"));
    // the output line is "package:/data/data/[…]/base.apk"
    let path = &output.stdout()[8..];
    let args = AdbArgs::spawn(&[PULL, path, destination.to_str()]);
    let output = run_adb_for(args, device.serial)?;
    if output.success() {
        SAVED.println_formatted(&[&destination.to_string()]);
    }
    return output.to_rslt()
}

pub fn run_apk(apk: String, config: &Config)-> Rslt<()> {
    if apk.is_empty() {
        return NO_PATH.to_err()
    }
    if !Path::new(&apk).exists() {
        return NO_FILE.to_err()
    }
    let aapt = get_aapt(&config)?;
    let device = resolve_device()?;
    let output = install(&device, &apk)?;
    if !output.success() {
        return output.to_rslt()
    }
    let (package, activity) = get_package_activity(aapt, &apk)?;
    let output = launch(device, package, activity)?;
    return output.to_rslt()
}

fn get_aapt(config: &Config) -> Rslt<PathBuf> {
    let path = match config.build_tools() {
        None => return NO_ANDROID_SDK.formatted(&[&config_path().to_string()]).to_err(),
        Some(path) => path,
    };
    let pattern = Regex::new(r"^\d+\.\d+\.\d+$")?;
    let path = fs::read_dir(&path)
        .string_err()?
        .filter_map(Result::ok)
        .filter(|it| it.metadata().map(|m| m.is_dir()).unwrap_or(false) && pattern.is_match(&it.file_name().to_string_lossy()))
        .map(|it| it.path())
        .max()
        .map(|it| it.join("aapt"))
        .ok_or_else(|| NO_BUILD_TOOLS.formatted(&[&path.to_string()]))?;
    return Ok(PathBuf::from(path));
}

fn install(device: &AdbDevice, apk: &String) -> Rslt<Output> {
    let args = AdbArgs::spawn(&[INSTALL, apk.as_str()]);
    return run_adb_for(args, device.serial.clone());
}

fn get_package_activity(aapt: PathBuf, apk: &String) -> Rslt<(String, String)> {
    let text = Command::new(aapt)
        .arg("d").arg("xmltree").arg(apk).arg("AndroidManifest.xml")
        .output()?
        .convert()
        .stdout()
        .replace('\n', " ");
    let package = Regex::new(r#" A: package="[^"]+"#)?;
    let package = package.find(&text)
        .ok_or_else(|| NO_PACKAGE_FOUND.value())?
        .as_str();
    let offset = package.index_of('"')
        .ok_or_else(|| NO_QUOTE_FOUND.value())? + 1;
    let package = package[offset..].to_string();
    let activity = Regex::new(r#" E: activity.+="android\.intent\.action\.MAIN""#)?;
    let activity = activity.find(&text)
        .ok_or_else(|| NO_ACTIVITY_FOUND.value())?.as_str();
    let name = Regex::new(r#" A: android:name\(0x\d{8}\)="[^"]+"#)?;
    let activity = name.find(activity)
        .ok_or_else(|| NO_ACTIVITY_NAME_FOUND.value())?.as_str();
    let offset = activity.index_of('"')
        .ok_or_else(|| NO_QUOTE_FOUND.value())? + 1;
    let activity = activity[offset..].to_string();
    return Ok((package, activity));
}

fn launch(device: AdbDevice, package: String, activity: String) -> Rslt<Output> {
    let command = format!("am start -a android.intent.action.MAIN -n {package}/{activity}");
    let args = AdbArgs::spawn(&[SHELL, command.as_str()]);
    return run_adb_for(args, device.serial);
}
