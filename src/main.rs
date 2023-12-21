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
    RunApk(String),
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
        Feature::RunApk(apk) => run_apk(apk),
        Feature::LastScreenShots(params) => pull_screenshots(params),
        Feature::LastScreenCasts(params) => pull_screencasts(params),
        Feature::MakeScreenShot(dst) => make_screenshot(dst),
        Feature::MakeScreenCast(dst) => make_screencast(dst),
        Feature::StealApk(package, dst) => steal_apk(package, dst),
    }
}

fn resolve_feature() -> Result<Feature, String> {
    let args = args().collect::<Vec<String>>();
    let mut arg_index = 1;
    let first = args[0].clone();
    let mut command = first.as_str();
    if ["adb", "adb-ext"].contains(&command) {
        arg_index += 1;
        command = args.get(1)
            .map(|it| it.as_str())
            .unwrap_or("");
    }
    let feature = match () {
        _ if command == "" => Feature::RunAdbWithArgs,
        _ if command == "lss" => Feature::LastScreenShots(get_params(args.get(arg_index).cloned())),
        _ if command == "lsc" => Feature::LastScreenCasts(get_params(args.get(arg_index).cloned())),
        _ if command == "mss"
            || command == "shot" => Feature::MakeScreenShot(args.get(arg_index).cloned().unwrap_or(String::new())),
        _ if command == "msc"
            || command == "rec" => Feature::MakeScreenCast(args.get(arg_index).cloned().unwrap_or(String::new())),
        _ if command == ARG_FIX => Feature::FixPermission(args.get(arg_index).cloned()),
        _ if command == "run" => Feature::RunApk(args[arg_index].clone()),
        _ if command == "steal" => Feature::StealApk(
            args.get(arg_index).expect("No package name passed").clone(),
            args.get(arg_index + 1).cloned(),
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
