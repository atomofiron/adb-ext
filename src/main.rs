use crate::core::apks::{run_apk, steal_apk};
use crate::core::config::Config;
use crate::core::fix::fix_on_linux;
use crate::core::layout_bounds::debug_layout_bounds;
use crate::core::orientation::{orientation, Orientation};
use crate::core::pull_media::{pull_screencasts, pull_screenshots, Params};
use crate::core::r#const::*;
use crate::core::screencap::make_screenshot;
use crate::core::screenrecord::make_screencast;
use crate::core::selector::resolve_device_and_run_args;
use crate::core::strings::{Language, DONE_ANOTHER_ONE, INPUT_PARAMETERS_OR_PRESS_ENTER, NO_PACKAGE_NAME};
#[cfg(windows)]
use crate::core::system::DOT_EXE;
use crate::core::updater::{deploy, update};
use crate::core::util::{get_help, print_the_fuck_out, println, set_sdk, string};
use std::env;
use std::env::args;
use std::io::stdin;
use std::path::Path;
use crate::core::pointer::toggle_pointer;
use crate::core::system::{adb_name, bin_name, ADB_EXT};
use crate::core::taps::toggle_taps;

mod core;
mod tests;

pub const ARG_FIX: &str = "fix";

enum StartName {
    Adb,
    AdbExt,
    None,
}

enum Feature {
    FixPermission(Option<String>),
    RunAdbWithArgs,
    RunApk(String),
    StealApk(String,Option<String>),
    LastScreenShots(Params),
    LastScreenCasts(Params),
    MakeScreenShot(String,String),
    MakeScreenCast(String,String),
    Deploy,
    Update,
    Orientation(Orientation),
    LayoutBounds,
    Touches,
    Pointer,
    Sdk(Option<String>),
}

fn main() {
    if let Ok(true) = env::var("LANG").map(|lang| lang.starts_with("ru")) {
        Language::set_language(Language::Ru);
    }
    let mut config = Config::read();
    config.write();
    config.update_adb_path();
    let mut args = args().collect::<Vec<String>>();
    let name = start_name(args.get(0));
    if !matches!(name, StartName::None) {
        args.remove(0);
    }
    if args.is_empty() && matches!(name, StartName::AdbExt) {
        println(&get_help(None));
        INPUT_PARAMETERS_OR_PRESS_ENTER.println();
        looped_work(&mut config);
    } else {
        work(args, &mut config);
    }
}

fn looped_work(config: &mut Config) {
    print!("{ADB_EXT}> ");
    print_the_fuck_out();
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let trimmed = line.trim();
    if !trimmed.is_empty() {
        let args = shell_words::split(trimmed)
            .unwrap();
        if work(args, config) {
            DONE_ANOTHER_ONE.println();
        }
        looped_work(config);
    }
}

fn work(args: Vec<String>, config: &mut Config) -> bool {
    let feature = match match_arg(args) {
        Some(feature) => feature,
        None => return false,
    };
    match feature {
        Feature::FixPermission(serial) => fix_on_linux(serial),
        Feature::RunAdbWithArgs => resolve_device_and_run_args(),
        Feature::RunApk(apk) => run_apk(apk, config),
        Feature::LastScreenShots(params) => pull_screenshots(params, config),
        Feature::LastScreenCasts(params) => pull_screencasts(params, config),
        Feature::MakeScreenShot(cmd, dst) => make_screenshot(cmd, dst, config),
        Feature::MakeScreenCast(cmd, dst) => make_screencast(cmd, dst, config),
        Feature::StealApk(package, dst) => steal_apk(package, dst),
        Feature::Deploy => deploy(),
        Feature::Update => update(),
        Feature::Orientation(param) => orientation(param),
        Feature::LayoutBounds => debug_layout_bounds(),
        Feature::Touches => toggle_taps(),
        Feature::Pointer => toggle_pointer(),
        Feature::Sdk(path) => set_sdk(path, config),
    }
    return true;
}

fn start_name(value: Option<&String>) -> StartName {
    let value = match value {
        None => return StartName::None,
        Some(value) => value,
    };
    let trimmed = value.trim_matches(['"', '\'']);
    let name = Path::new(trimmed)
        .file_name()
        .and_then(|os| os.to_str())
        .unwrap_or(trimmed)
        .to_ascii_lowercase();
    #[cfg(unix)]
    let base = name.as_str();
    #[cfg(windows)]
    let base = name.strip_suffix(DOT_EXE).unwrap_or(&name);
    return match () {
        _ if base == adb_name() => StartName::Adb,
        _ if base == bin_name() => StartName::AdbExt,
        _ => StartName::None,
    }
}

fn match_arg(args: Vec<String>) -> Option<Feature> {
    let first = args.get(0)
        .unwrap_or(&string(""))
        .to_ascii_lowercase();
    let feature = match first.as_str() {
        "" => Feature::RunAdbWithArgs,
        LSS => Feature::LastScreenShots(Params::from(first, args.get(1).cloned())),
        LSC => Feature::LastScreenCasts(Params::from(first, args.get(1).cloned())),
        MSS | SHOT => Feature::MakeScreenShot(first, args.get(1).cloned().unwrap_or(String::new())),
        MSC | REC | RECORD => Feature::MakeScreenCast(first, args.get(1).cloned().unwrap_or(String::new())),
        ARG_FIX => Feature::FixPermission(args.get(1).cloned()),
        RUN => Feature::RunApk(args.get(1).cloned().unwrap_or(String::new())),
        STEAL => Feature::StealApk(
            args.get(1).expect(NO_PACKAGE_NAME.value()).clone(),
            args.get(2).cloned(),
        ),
        DEPLOY => Feature::Deploy,
        UPDATE => Feature::Update,
        PORT => Feature::Orientation(Orientation::portrait(false)),
        LAND => Feature::Orientation(Orientation::landscape(false)),
        FPORT => Feature::Orientation(Orientation::portrait(true)),
        FLAND => Feature::Orientation(Orientation::landscape(true)),
        ACCEL => Feature::Orientation(Orientation::accelerometer(true)),
        NOACCEL => Feature::Orientation(Orientation::accelerometer(false)),
        BOUNDS => Feature::LayoutBounds,
        TAPS => Feature::Touches,
        POINTER => Feature::Pointer,
        SDK => Feature::Sdk(args.get(1).cloned()),
        "shit" => {
            println!("💩");
            return None
        },
        _ => Feature::RunAdbWithArgs,
    };
    return Some(feature)
}
