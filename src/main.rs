use crate::core::ext::ShortUnwrap;
use crate::core::pull_media::{Params, pull_screencasts, pull_screenshots};
use crate::core::selector::resolve_device_and_run_args;
use crate::core::strings::{Language, LINUX_ONLY};
use crate::core::usb_resolver::resolve_permission;
use std::env;
use std::env::args;
use crate::core::make_screenshot::make_screenshot;

mod core;

pub const ARG_FIX: &str = "fix";

enum Feature {
    FixPermission(Option<String>),
    RunAdbWithArgs,
    LastScreenShots(Params),
    LastScreenCasts(Params),
    MakeScreenShot(String),
}

fn main() {
    if env::var("LANG")
        .map(|lang| lang.starts_with("ru"))
        .unwrap_or(false)
    {
        Language::set_language(Language::Ru)
    }
    match resolve_feature().short_unwrap() {
        Feature::FixPermission(serial) => resolve_permission(serial),
        Feature::RunAdbWithArgs => resolve_device_and_run_args(),
        Feature::LastScreenShots(params) => pull_screenshots(params),
        Feature::LastScreenCasts(params) => pull_screencasts(params),
        Feature::MakeScreenShot(dst) => make_screenshot(dst),
    }
}

fn resolve_feature() -> Result<Feature, String> {
    let args = args().collect::<Vec<String>>();
    let feature = match () {
        _ if args[0] == "lss" => Feature::LastScreenShots(get_params(args.get(2))),
        _ if args[0] == "lsc" => Feature::LastScreenCasts(get_params(args.get(2))),
        _ if args[0] == "mss" || args[0] == "shot" => Feature::MakeScreenShot(args.get(2).cloned().unwrap_or(String::new())),
        _ if args.len() > 1 && args[1] == ARG_FIX => match () {
            _ if cfg!(target_os = "linux") => {
                Feature::FixPermission(args.get(2).cloned())
            },
            _ => return Err(LINUX_ONLY.value().to_string()),
        },
        _ => Feature::RunAdbWithArgs,
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
