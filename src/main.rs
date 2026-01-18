use crate::core::ext::ShortUnwrap;
use crate::core::pull_media::{Params, pull_screencasts, pull_screenshots};
use crate::core::selector::resolve_device_and_run_args;
use crate::core::strings::{Language, NO_PACKAGE_NAME};
use std::env;
use std::env::args;
use crate::core::screencap::make_screenshot;
use crate::core::fix::fix_on_linux;
use crate::core::apks::{run_apk, steal_apk};
use crate::core::config::Config;
use crate::core::layout_bounds::debug_layout_bounds;
use crate::core::orientation::{orientation, Orientation};
use crate::core::r#const::*;
use crate::core::screenrecord::make_screencast;
use crate::core::updater::{deploy, update};
use crate::core::util::set_sdk;

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
    Skd(Option<String>),
}

fn main() {
    if let Ok(true) = env::var("LANG").map(|lang| lang.starts_with("ru")) {
        Language::set_language(Language::Ru);
    }
    let config = Config::read();
    config.update_adb_path();
    match resolve_feature().short_unwrap() {
        Feature::FixPermission(serial) => fix_on_linux(serial),
        Feature::RunAdbWithArgs => resolve_device_and_run_args(),
        Feature::RunApk(apk) => run_apk(apk, &config),
        Feature::LastScreenShots(params) => pull_screenshots(params, config),
        Feature::LastScreenCasts(params) => pull_screencasts(params, config),
        Feature::MakeScreenShot(cmd,dst) => make_screenshot(cmd, dst, &config),
        Feature::MakeScreenCast(cmd,dst) => make_screencast(cmd, dst, &config),
        Feature::StealApk(package, dst) => steal_apk(package, dst),
        Feature::Deploy => deploy(),
        Feature::Update => update(),
        Feature::Orientation(param) => orientation(param),
        Feature::LayoutBounds => debug_layout_bounds(),
        Feature::Skd(path) => set_sdk(path, config),
    }
}

fn resolve_feature() -> Result<Feature, String> {
    let args = args().collect::<Vec<String>>();
    let first = vec![Some(args[0].clone()), args.get(1).cloned()]
        .into_iter().filter_map(|it| it)
        .collect::<Vec<String>>();
    let mut feature = Feature::RunAdbWithArgs;
    for (i, arg) in first.iter().enumerate() {
        feature = match_arg(arg.to_string().to_lowercase(), args.clone(), i + 1);
        if !matches!(feature, Feature::RunAdbWithArgs) {
            break
        }
    }
    return Ok(feature);
}

fn match_arg(cmd: String, args: Vec<String>, next: usize) -> Feature {
    match () {
        _ if cmd == "" => Feature::RunAdbWithArgs,
        _ if cmd == LSS => Feature::LastScreenShots(Params::from(cmd, args.get(next).cloned())),
        _ if cmd == LSC => Feature::LastScreenCasts(Params::from(cmd, args.get(next).cloned())),
        _ if cmd == MSS
            || cmd == SHOT => Feature::MakeScreenShot(cmd, args.get(next).cloned().unwrap_or(String::new())),
        _ if cmd == MSC
            || cmd == REC
            || cmd == RECORD => Feature::MakeScreenCast(cmd, args.get(next).cloned().unwrap_or(String::new())),
        _ if cmd == ARG_FIX => Feature::FixPermission(args.get(next).cloned()),
        _ if cmd == RUN => Feature::RunApk(args.get(next).cloned().unwrap_or(String::new())),
        _ if cmd == STEAL => Feature::StealApk(
            args.get(next).expect(NO_PACKAGE_NAME.value()).clone(),
            args.get(next + 1).cloned(),
        ),
        _ if cmd == DEPLOY => Feature::Deploy,
        _ if cmd == UPDATE => Feature::Update,
        _ if cmd == PORT => Feature::Orientation(Orientation::portrait(false)),
        _ if cmd == LAND => Feature::Orientation(Orientation::landscape(false)),
        _ if cmd == FPORT => Feature::Orientation(Orientation::portrait(true)),
        _ if cmd == FLAND => Feature::Orientation(Orientation::landscape(true)),
        _ if cmd == ACCEL => Feature::Orientation(Orientation::accelerometer(true)),
        _ if cmd == NOACCEL => Feature::Orientation(Orientation::accelerometer(false)),
        _ if cmd == BOUNDS => Feature::LayoutBounds,
        _ if cmd == SDK => Feature::Skd(args.get(next).cloned()),
        _ => Feature::RunAdbWithArgs,
    }
}
