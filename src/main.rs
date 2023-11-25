use crate::core::ext::ShortUnwrap;
use crate::core::pull_media::{Params, pull_screencasts, pull_screenshots};
use crate::core::selector::resolve_device_and_run_args;
use crate::core::strings::{Language, LINUX_ONLY, UNKNOWN_COMMAND};
use crate::core::usb_resolver::resolve_permission;
use std::env;
use std::env::args;
use crate::core::make_screenshot::make_screenshot;

mod core;

const ENV_LANG: &str = "LANG";
const RU: &str = "ru";

enum Feature {
    FixPermission,
    SelectDevice,
    LastScreenShots(Params),
    LastScreenCasts(Params),
    MakeScreenShot(String),
}

fn main() {
    if env::var(ENV_LANG)
        .map(|lang| lang.starts_with(RU))
        .unwrap_or(false)
    {
        Language::set_language(Language::Ru)
    }
    match resolve_feature().short_unwrap() {
        Feature::FixPermission => resolve_permission(),
        Feature::SelectDevice => resolve_device_and_run_args(),
        Feature::LastScreenShots(params) => pull_screenshots(params),
        Feature::LastScreenCasts(params) => pull_screencasts(params),
        Feature::MakeScreenShot(dst) => make_screenshot(dst),
    }
}

fn resolve_feature() -> Result<Feature, String> {
    let args = args().collect::<Vec<String>>();
    let feature = match () {
        _ if args[0] == "adb-ext" && args.len() > 1 && args[1] == "fix" => match () {
            _ if cfg!(target_os = "linux") => Feature::FixPermission,
            _ => return Err(LINUX_ONLY.value().to_string()),
        },
        _ if args[0] == "adb-ext" => Feature::SelectDevice,
        _ if args[0] == "lss" => Feature::LastScreenShots(get_params(args.get(2))),
        _ if args[0] == "lsc" => Feature::LastScreenCasts(get_params(args.get(2))),
        _ if args[0] == "mss" || args[0] == "shot" => Feature::MakeScreenShot(args.get(2).cloned().unwrap_or(String::new())),
        _ => return Err(UNKNOWN_COMMAND.value().to_string()),
    };
    return Ok(feature);
}

fn get_params(arg: Option<&String>) -> Params {
    arg.map_or_else(
        || Params::Single(String::new()),
        |it| it.parse::<usize>().map_or(
            Params::Single(it.clone()),
            |it| Params::Count(it),
        ),
    )
}
