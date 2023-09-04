use std::env::args;
use std::env;
use crate::core::lss::pull_screenshots;
use crate::core::pain::resolve_permission;
use crate::core::selector::run_with_device;
use crate::core::strings::{Language, UNKNOWN_COMMAND};
use crate::core::util::ShortUnwrap;

mod core;

const ENV_LANG: &str = "LANG";
const RU: &str = "ru";
const ADB: &str = "adb";
const LSS: &str = "lss";

enum Feature {
    FixPermission, SelectDevice, LastScreenShots(u32)
}

fn main() {
    if env::var(ENV_LANG)
        .map(|lang| lang.starts_with(RU))
        .unwrap_or(false) {
        Language::set_language(Language::Ru)
    }
    match resolve_feature().short_unwrap() {
        Feature::FixPermission => resolve_permission(),
        Feature::SelectDevice => run_with_device(),
        Feature::LastScreenShots(count) => pull_screenshots(count),
    }
}

fn resolve_feature() -> Result<Feature, String> {
    let args = args().collect::<Vec<String>>();
    let feature = match () {
        _ if args.len() == 1 => Feature::FixPermission,
        _ if args[1] == ADB => Feature::SelectDevice,
        _ if args[1] == LSS => Feature::LastScreenShots(get_count(args.get(2))),
        _ => return Err(String::from(UNKNOWN_COMMAND.value())),
    };
    return Ok(feature)
}

fn get_count(arg: Option<&String>) -> u32 {
    arg.map(|it| it.parse::<u32>())
        .unwrap_or(Ok(1))
        .unwrap_or(1)
}
