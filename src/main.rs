use crate::core::ext::ShortUnwrap;
use crate::core::lss::pull_screenshots;
use crate::core::selector::resolve_device_and_run_args;
use crate::core::strings::{Language, LINUX_ONLY, UNKNOWN_COMMAND};
use crate::core::usb_resolver::resolve_permission;
use std::env;
use std::env::args;

mod core;

const ENV_LANG: &str = "LANG";
const RU: &str = "ru";
const ADB: &str = "adb";
const LSS: &str = "lss";

enum Feature {
    FixPermission,
    SelectDevice,
    LastScreenShots(usize),
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
        Feature::LastScreenShots(count) => pull_screenshots(count),
    }
}

fn resolve_feature() -> Result<Feature, String> {
    let args = args().collect::<Vec<String>>();
    let feature = match () {
        _ if args.len() <= 1 && cfg!(target_os = "linux") => Feature::FixPermission,
        _ if args.len() <= 1 => return Err(String::from(LINUX_ONLY.value())),
        _ if args[1] == ADB => Feature::SelectDevice,
        _ if args[1] == LSS => Feature::LastScreenShots(get_count(args.get(2))),
        _ => return Err(String::from(UNKNOWN_COMMAND.value())),
    };
    return Ok(feature);
}

fn get_count(arg: Option<&String>) -> usize {
    arg.map(|it| it.parse::<usize>())
        .unwrap_or(Ok(1))
        .unwrap_or(1)
}
