use crate::core::adb_command::AdbArgs;
use crate::core::adb_device::AdbDevice;
use crate::core::config::Config;
use crate::core::destination::Destination;
use crate::core::ext::{OutputExt, PathBufExt, PrintExt, ResultExt, StrExt};
use crate::core::r#const::{INSTALL, PULL, SHELL};
use crate::core::selector::{resolve_device, run_adb_with};
use crate::core::strings::{NO_ANDROID_SDK, NO_BUILD_TOOLS, NO_FILE, NO_PATH, SAVED};
use crate::core::system::config_path;
use crate::core::util::string;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Output};

pub fn steal_apk(package: String, dst: Option<String>) -> ExitCode {
    let pm_command = format!("pm path {package}");
    let args = AdbArgs::run(&[SHELL, pm_command.as_str()]);
    let device = match resolve_device() {
        Ok(device) => device,
        Err(code) => return code,
    };
    let output = run_adb_with(&device, args);
    if !output.status.success() {
        output.print_err();
        return output.exit_code()
    }
    let destination = dst
        .unwrap_or(string(""))
        .dst()
        .join(format!("{package}.apk"));
    // the output line is "package:/data/data/[…]/base.apk"
    let path = &output.stdout().clone()[8..];
    let args = AdbArgs::spawn(&[PULL, path, destination.to_str().unwrap()]);
    let output = run_adb_with(&device, args);
    if output.status.success() {
        SAVED.println_formatted(&[&destination.to_string()]);
    }
    return output.exit_code()
}

pub fn run_apk(apk: String, config: &Config)-> ExitCode {
    if apk.is_empty() {
        NO_PATH.eprintln();
        return ExitCode::FAILURE;
    }
    if !Path::new(&apk).exists() {
        NO_FILE.eprintln();
        return ExitCode::FAILURE;
    }
    let aapt = match get_aapt(&config) {
        Ok(path) => path,
        Err(err) => {
            err.eprintln();
            return ExitCode::FAILURE
        },
    };
    let device = match resolve_device() {
        Ok(device) => device,
        Err(code) => return code,
    };
    let output = install(&device, &apk);
    if !output.status.success() {
        return output.exit_code()
    }
    let (package, activity) = get_package_activity(aapt, &apk);
    let output = launch(&device, package, activity);
    return output.exit_code()
}

fn get_aapt(config: &Config) -> Result<PathBuf, String> {
    let path = match config.build_tools() {
        None => return Err(NO_ANDROID_SDK.formatted(&[&config_path().to_string()])),
        Some(path) => path,
    };
    let pattern = Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
    return fs::read_dir(&path)
        .string_err()?
        .filter_map(Result::ok)
        .filter(|it| pattern.is_match(&it.file_name().to_string_lossy()))
        .map(|it| it.path())
        .max()
        .map(|it| it.join("aapt"))
        .ok_or_else(|| NO_BUILD_TOOLS.formatted(&[&path.to_string()]));
}

fn install(device: &AdbDevice, apk: &String) -> Output {
    let args = AdbArgs::spawn(&[INSTALL, apk.as_str()]);
    return run_adb_with(&device, args);
}

fn get_package_activity(aapt: PathBuf, apk: &String) -> (String, String) {
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
