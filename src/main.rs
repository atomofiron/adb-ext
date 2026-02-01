use crate::core::anim_scale::change_anim_scale;
use crate::core::apks::{run_apk, steal_apk};
use crate::core::cmd_editor::{CmdEditor, CmdHelper, CmdHighlight};
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
use crate::core::start_mode::StartMode;
use crate::core::strings::{Language, INPUT_OR_EXIT};
#[cfg(windows)]
use crate::core::system::DOT_EXE;
use crate::core::system::{history_path, ADB_EXT};
use crate::core::taps::toggle_taps;
use crate::core::updater::{deploy, update};
use crate::core::util::{get_help, print_version, string};
use rustyline::error::ReadlineError;
use std::cell::RefCell;
use std::env;
use std::env::args;
use std::path::Path;
use std::process::ExitCode;
use std::rc::Rc;

mod core;
mod tests;

fn main() -> ExitCode {
    if let Ok(true) = env::var("LANG").map(|lang| lang.starts_with("ru")) {
        Language::set_language(Language::Ru);
    }
    let mut config = Config::read();
    config.write().unwrap();
    config.update_adb_path();
    let mut args = args().collect::<Vec<String>>();
    let mode = start_name(args.get(0));
    if !matches!(mode, StartMode::Unknown) {
        args.remove(0);
    }
    return if args.is_empty() && matches!(mode, StartMode::AdbExt) {
        INPUT_OR_EXIT.println();
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
        work(mode, args, &mut config)
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
                    Ok(args) => code = Some(work(StartMode::AdbExt, args, config)),
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

fn work(mode: StartMode, args: Vec<String>, config: &mut Config) -> ExitCode {
    let first = args.get(0)
        .unwrap_or(&string(""))
        .to_ascii_lowercase();
    match first.as_str() {
        LSS => return pull_screenshots(Params::from(first, args.get(1).cloned()), config),
        LSC => return pull_screencasts(Params::from(first, args.get(1).cloned()), config),
        MSS | SHOT => return make_screenshot(first, args.get(1).cloned().unwrap_or_default(), config),
        MSC | REC | RECORD => return make_screencast(first, args.get(1).cloned().unwrap_or_default(), config),
        FIX => return fix_on_linux(args.get(1).cloned()),
        RUN => return run_apk(args.get(1).cloned().unwrap_or_default(), config),
        STEAL => return steal_apk(args.get(1).cloned(), args.get(2).cloned()),
        DEPLOY => return deploy(),
        UPDATE => return update(),
        PORT => return orientation(Orientation::portrait(false)),
        LAND => return orientation(Orientation::landscape(false)),
        FPORT => return orientation(Orientation::portrait(true)),
        FLAND => return orientation(Orientation::landscape(true)),
        ACCEL => return orientation(Orientation::accelerometer(true)),
        NO_ACCEL => return orientation(Orientation::accelerometer(false)),
        BOUNDS => return debug_layout_bounds(),
        TAPS => return toggle_taps(),
        POINTER => return toggle_pointer(),
        ANI_SCALE => return change_anim_scale(args.get(1).cloned().unwrap_or_default()),
        SDK => return set_sdk(args.get(1).cloned(), config),
        VERSION if !mode.adb() => print_version(),
        HELP if !mode.adb() => get_help(None).println(),
        "shit" => "💩".println(),
        _ => return resolve_device_and_run_args(args.as_slice()),
    };
    return ExitCode::SUCCESS
}

fn start_name(value: Option<&String>) -> StartMode {
    let value = match value {
        None => return StartMode::Unknown,
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
        _ if base == ADB => StartMode::Adb,
        _ if base == ADB_EXT => StartMode::AdbExt,
        _ => StartMode::Unknown,
    }
}
