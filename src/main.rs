use crate::core::ext::{OptionExt, ResultToOption, ShortUnwrap};
use crate::core::pull_media::{Params, pull_screencasts, pull_screenshots};
use crate::core::selector::resolve_device_and_run_args;
use crate::core::strings::Language;
use std::env;
use std::env::args;
use crate::core::screencap::make_screenshot;
use crate::core::fix::fix_on_linux;
use crate::core::apks::{run_apk, steal_apk};
use crate::core::screenrecord::make_screencast;
use crate::core::updater::{deploy, update};

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
        Feature::MakeScreenShot(cmd,dst) => make_screenshot(cmd, dst),
        Feature::MakeScreenCast(cmd,dst) => make_screencast(cmd, dst),
        Feature::StealApk(package, dst) => steal_apk(package, dst),
        Feature::Deploy => deploy(),
        Feature::Update => update(),
    }
}

fn resolve_feature() -> Result<Feature, String> {
    let args = args().collect::<Vec<String>>();
    let first = vec![Some(args[0].clone()), args.get(1).cloned()]
        .into_iter().filter_map(|it| it)
        .collect::<Vec<String>>();
    let mut feature = Feature::RunAdbWithArgs;
    for (i, arg) in first.iter().enumerate() {
        feature = match_arg(arg.to_string(), args.clone(), i + 1);
        if !matches!(feature, Feature::RunAdbWithArgs) {
            break
        }
    }
    return Ok(feature);
}
fn match_arg(cmd: String, args: Vec<String>, next: usize) -> Feature {
    match () {
        _ if cmd == "" => Feature::RunAdbWithArgs,
        _ if cmd == "lss" => Feature::LastScreenShots(get_params(cmd, args.get(next).cloned())),
        _ if cmd == "lsc" => Feature::LastScreenCasts(get_params(cmd, args.get(next).cloned())),
        _ if cmd == "mss"
            || cmd == "shot" => Feature::MakeScreenShot(cmd, args.get(next).cloned().unwrap_or(String::new())),
        _ if cmd == "msc"
            || cmd == "rec" => Feature::MakeScreenCast(cmd, args.get(next).cloned().unwrap_or(String::new())),
        _ if cmd == ARG_FIX => Feature::FixPermission(args.get(next).cloned()),
        _ if cmd == "run" => Feature::RunApk(args[next].clone()),
        _ if cmd == "steal" => Feature::StealApk(
            args.get(next).expect("No package name passed").clone(),
            args.get(next + 1).cloned(),
        ),
        _ if cmd == "deploy" => Feature::Deploy,
        _ if cmd == "update" => Feature::Update,
        _ => Feature::RunAdbWithArgs,
    }
}

fn get_params(cmd: String, arg: Option<String>) -> Params {
    match arg.flat_map(|it| it.parse::<usize>().to_option()) {
        Some(count) => Params::Count(cmd, count),
        None => Params::Single(cmd, arg),
    }
}
