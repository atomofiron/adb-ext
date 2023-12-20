use crate::core::ext::ShortUnwrap;
use crate::core::pull_media::{Params, pull_screencasts, pull_screenshots};
use crate::core::selector::resolve_device_and_run_args;
use crate::core::strings::Language;
use std::env;
use std::env::args;
use crate::core::screencap::make_screenshot;
use crate::core::fix::fix_on_linux;
use crate::core::apks::{run_apk, steal_apk};
use crate::core::screenrecord::make_screencast;

mod core;
mod tests;

pub const ARG_FIX: &str = "fix";

enum Feature {
    FixPermission(Option<String>),
    RunAdbWithArgs,
    RunApk,
    StealApk(String,Option<String>),
    LastScreenShots(Params),
    LastScreenCasts(Params),
    MakeScreenShot(String),
    MakeScreenCast(String), // todo Ctrl-C
}

fn main() {
    if let Ok(true) = env::var("LANG").map(|lang| lang.starts_with("ru")) {
        Language::set_language(Language::Ru);
    }
    match resolve_feature().short_unwrap() {
        Feature::FixPermission(serial) => fix_on_linux(serial),
        Feature::RunAdbWithArgs => resolve_device_and_run_args(),
        Feature::RunApk => run_apk(),
        Feature::LastScreenShots(params) => pull_screenshots(params),
        Feature::LastScreenCasts(params) => pull_screencasts(params),
        Feature::MakeScreenShot(dst) => make_screenshot(dst),
        Feature::MakeScreenCast(dst) => make_screencast(dst),
        Feature::StealApk(package, dst) => steal_apk(package, dst),
    }
}

fn resolve_feature() -> Result<Feature, String> {
    let args = args().collect::<Vec<String>>();
    let feature = match () {
        _ if args[0] == "lss" => Feature::LastScreenShots(get_params(args.get(1).cloned())),
        _ if args[0] == "lsc" => Feature::LastScreenCasts(get_params(args.get(1).cloned())),
        _ if args[0] == "mss" || args[0] == "shot" => Feature::MakeScreenShot(args.get(1).cloned().unwrap_or(String::new())),
        _ if args[0] == "msc" || args[0] == "rec" => Feature::MakeScreenCast(args.get(1).cloned().unwrap_or(String::new())),
        _ if args.len() == 1 => Feature::RunAdbWithArgs,
        _ if args[1] == ARG_FIX => Feature::FixPermission(args.get(2).cloned()),
        _ if args[1] == "run" => Feature::RunApk,
        _ if args[1] == "steal" => Feature::StealApk(
            args.get(2).expect("No package name passed").clone(),
            args.get(3).cloned(),
        ),
        _ => Feature::RunAdbWithArgs,
    };
    return Ok(feature);
}

fn get_params(arg: Option<String>) -> Params {
    arg.map_or_else(
        || Params::Single(String::new()),
        |it| it.parse::<usize>().map_or(
            Params::Single(it),
            |it| Params::Count(it),
        ),
    )
}
