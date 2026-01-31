use crate::core::anim_scale::change_anim_scale;
use crate::core::apks::{run_apk, steal_apk};
use crate::core::completer::{CmdEditor, CmdHelper, CmdHighlight};
use crate::core::config::Config;
use crate::core::ext::{PrintExt, ResultExt};
use crate::core::fix::fix_on_linux;
use crate::core::layout_bounds::debug_layout_bounds;
use crate::core::orientation::{orientation, Orientation};
use crate::core::pointer::toggle_pointer;
use crate::core::pull_media::{pull_screencasts, pull_screenshots, Params};
use crate::core::r#const::*;
use crate::core::screencap::make_screenshot;
use crate::core::screenrecord::make_screencast;
use crate::core::sdk::set_sdk;
use crate::core::selector::resolve_device_and_run_args;
use crate::core::strings::{Language, INPUT_PARAMETERS_OR_EXIT};
#[cfg(windows)]
use crate::core::system::DOT_EXE;
use crate::core::system::{bin_name, history_path, ADB_EXT};
use crate::core::taps::toggle_taps;
use crate::core::updater::{deploy, update};
use crate::core::util::{get_help, string};
use rustyline::error::ReadlineError;
use std::cell::RefCell;
use std::env;
use std::env::args;
use std::path::Path;
use std::process::ExitCode;
use std::rc::Rc;

mod core;
mod tests;

enum StartName {
    Adb,
    AdbExt,
    None,
}

enum Feature {
    FixPermission(Option<String>),
    RunAdbWithArgs,
    RunApk(String),
    StealApk(Option<String>, Option<String>),
    LastScreenShots(Params),
    LastScreenCasts(Params),
    MakeScreenShot(String, String),
    MakeScreenCast(String, String),
    Deploy,
    Update,
    Orientation(Orientation),
    LayoutBounds,
    Touches,
    Pointer,
    AnimScale(String),
    Sdk(Option<String>),
}

fn main() -> ExitCode {
    if let Ok(true) = env::var("LANG").map(|lang| lang.starts_with("ru")) {
        Language::set_language(Language::Ru);
    }
    let mut config = Config::read();
    config.write().unwrap();
    config.update_adb_path();
    let mut args = args().collect::<Vec<String>>();
    let name = start_name(args.get(0));
    if !matches!(name, StartName::None) {
        args.remove(0);
    }
    return if args.is_empty() && matches!(name, StartName::AdbExt) {
        get_help(None).println();
        INPUT_PARAMETERS_OR_EXIT.println();
        let mut input = CmdEditor::new().unwrap();
        let success = Rc::new(RefCell::new(None));
        let helper = CmdHelper::from(SUGGESTIONS, success.clone());
        input.set_helper(Some(helper));
        let history_path = history_path();
        if history_path.exists() {
            input.load_history(&history_path).unwrap();
        }
        let code = looper_work(&mut input, &mut config, success);
        input.save_history(&history_path).unwrap();
        code
    } else {
        work(args, &mut config)
    }
}

fn looper_work(input: &mut CmdEditor, config: &mut Config, success: CmdHighlight) -> ExitCode {
    let mut code: Option<ExitCode> = None;
    loop {
        let previous = code.map(|code| code == ExitCode::SUCCESS);
        let status = match previous {
            None => string(""),
            Some(true) => string("✔ "),
            Some(false) => string("✘ "),
        };
        let status_range = 0..status.as_bytes().len();
        *success.borrow_mut() = previous.map(|success| (success, status_range));
        let prompt = format!("{status}{ADB_EXT}> ");
        match input.readline(&prompt) {
            Ok(line) => {
                let trimmed = line.trim();
                match trimmed {
                    "" => {
                        code = None;
                        continue
                    },
                    CLEAR => {
                        code = None;
                        input.clear_screen().soft_unwrap();
                        continue
                    },
                    EXIT | QUIT => break,
                    _ => (),
                }
                if !line.starts_with("  ") {
                    input.add_history_entry(trimmed).soft_unwrap();
                }
                match shell_words::split(trimmed) {
                    Ok(args) => code = Some(work(args, config)),
                    Err(e) => e.eprintln(),
                };
            }
            Err(ReadlineError::Interrupted) => { // Ctrl-C
                code = None;
                continue
            },
            Err(ReadlineError::Eof) => break, // Ctrl-D
            Err(e) => {
                e.eprintln();
                break;
            }
        }
    }
    return code.unwrap_or(ExitCode::SUCCESS);
}

fn work(args: Vec<String>, config: &mut Config) -> ExitCode {
    let feature = match match_arg(&args) {
        Some(feature) => feature,
        None => return ExitCode::SUCCESS,
    };
    return match feature {
        Feature::FixPermission(serial) => fix_on_linux(serial),
        Feature::RunAdbWithArgs => resolve_device_and_run_args(args.as_slice()),
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
        Feature::AnimScale(scale) => change_anim_scale(scale),
        Feature::Sdk(path) => set_sdk(path, config),
    }
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
        _ if base == ADB => StartName::Adb,
        _ if base == ADB_EXT => StartName::AdbExt,
        _ => StartName::None,
    }
}

fn match_arg(args: &Vec<String>) -> Option<Feature> {
    let first = args.get(0)
        .unwrap_or(&string(""))
        .to_ascii_lowercase();
    let feature = match first.as_str() {
        "" => Feature::RunAdbWithArgs,
        LSS => Feature::LastScreenShots(Params::from(first, args.get(1).cloned())),
        LSC => Feature::LastScreenCasts(Params::from(first, args.get(1).cloned())),
        MSS | SHOT => Feature::MakeScreenShot(first, args.get(1).cloned().unwrap_or_default()),
        MSC | REC | RECORD => Feature::MakeScreenCast(first, args.get(1).cloned().unwrap_or_default()),
        FIX => Feature::FixPermission(args.get(1).cloned()),
        RUN => Feature::RunApk(args.get(1).cloned().unwrap_or_default()),
        STEAL => Feature::StealApk(
            args.get(1).cloned(),
            args.get(2).cloned(),
        ),
        DEPLOY => Feature::Deploy,
        UPDATE => Feature::Update,
        PORT => Feature::Orientation(Orientation::portrait(false)),
        LAND => Feature::Orientation(Orientation::landscape(false)),
        FPORT => Feature::Orientation(Orientation::portrait(true)),
        FLAND => Feature::Orientation(Orientation::landscape(true)),
        ACCEL => Feature::Orientation(Orientation::accelerometer(true)),
        NO_ACCEL => Feature::Orientation(Orientation::accelerometer(false)),
        BOUNDS => Feature::LayoutBounds,
        TAPS => Feature::Touches,
        POINTER => Feature::Pointer,
        ANI_SCALE => Feature::AnimScale(args.get(1).cloned().unwrap_or_default()),
        SDK => Feature::Sdk(args.get(1).cloned()),
        "shit" => {
            "💩".println();
            return None
        },
        "--version" => {
            println!("{} v{}", bin_name(), env!("CARGO_PKG_VERSION"));
            return None
        }
        _ => Feature::RunAdbWithArgs,
    };
    return Some(feature)
}
