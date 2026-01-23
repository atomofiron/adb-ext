use crate::core::apks::{run_apk, steal_apk};
use crate::core::config::Config;
use crate::core::ext::ShortUnwrap;
use crate::core::fix::fix_on_linux;
use crate::core::layout_bounds::debug_layout_bounds;
use crate::core::orientation::{orientation, Orientation};
use crate::core::pull_media::{pull_screencasts, pull_screenshots, Params};
use crate::core::r#const::*;
use crate::core::screencap::make_screenshot;
use crate::core::screenrecord::make_screencast;
use crate::core::selector::resolve_device_and_run_args;
use crate::core::strings::{Language, NO_PACKAGE_NAME};
#[cfg(windows)]
use crate::core::system::DOT_EXE;
use crate::core::updater::{deploy, update};
use crate::core::util::{set_sdk, string};
use std::env;
use std::env::args;
use std::path::Path;
use crate::core::pointer::toggle_pointer;
use crate::core::taps::toggle_taps;

mod core;
mod tests;

pub const ARG_FIX: &str = "fix";

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
    let config = Config::read();
    config.write();
    config.update_adb_path();
    match resolve_feature().short_unwrap() {
        Feature::FixPermission(serial) => fix_on_linux(serial),
        Feature::RunAdbWithArgs => resolve_device_and_run_args(),
        Feature::RunApk(apk) => run_apk(apk, &config),
        Feature::LastScreenShots(params) => pull_screenshots(params, config),
        Feature::LastScreenCasts(params) => pull_screencasts(params, config),
        Feature::MakeScreenShot(cmd, dst) => make_screenshot(cmd, dst, &config),
        Feature::MakeScreenCast(cmd, dst) => make_screencast(cmd, dst, &config),
        Feature::StealApk(package, dst) => steal_apk(package, dst),
        Feature::Deploy => deploy(),
        Feature::Update => update(),
        Feature::Orientation(param) => orientation(param),
        Feature::LayoutBounds => debug_layout_bounds(),
        Feature::Touches => toggle_taps(),
        Feature::Pointer => toggle_pointer(),
        Feature::Sdk(path) => set_sdk(path, config),
    }
}

fn resolve_feature() -> Result<Feature, String> {
    let mut args = args().collect::<Vec<String>>();
    if is_bin_name(args.get(0)) {
        args.remove(0);
    }
    if args.is_empty() {
        return Ok(Feature::RunAdbWithArgs);
    }
    return match_arg(args)
}

fn is_bin_name(value: Option<&String>) -> bool {
    let value = match value {
        None => return false,
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
    matches!(base, "adb" | "adb-ext")
}

fn match_arg(args: Vec<String>) -> Result<Feature, String> {
    let first = args[0].to_ascii_lowercase();
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
        "shit" => return Err(string("💩")),
        _ => Feature::RunAdbWithArgs,
    };
    return Ok(feature)
}
