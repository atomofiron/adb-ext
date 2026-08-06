use self::core::tools::apks::{run_apk, steal_apk};
use self::core::tools::layout_bounds::debug_layout_bounds;
use self::core::tools::orientation::{Orientation, orientation};
use self::core::tools::pointer::toggle_pointer;
use self::core::tools::pull_media::{Params, pull_screencasts, pull_screenshots};
use self::core::tools::screencap::make_screenshot;
use self::core::tools::screenrecord::make_screencast;
use self::core::tools::taps::toggle_taps;
use self::core::util::adb_args::AdbArgs;
use self::core::util::cmd_editor::{CmdEditor, CmdHelper, CmdHighlight};
use self::core::util::config::Config;
use self::core::util::r#const::*;
use self::core::util::sdk::set_sdk;
use self::core::util::selector::run_adb;
use self::core::util::start_mode::StartMode;
use self::core::util::strings::{INPUT_OR_EXIT, Language};
#[cfg(windows)]
use self::core::util::system::DOT_EXE;
use self::core::util::system::history_path;
use self::core::util::updater::{deploy, update};
use crate::core::ext::Rslt;
use crate::core::ext::error_code::ErrorCode;
use crate::core::ext::print::PrintExt;
use crate::core::ext::result::ResultExt;
use crate::core::ext::user_cancelled::UserCancelled;
use crate::core::fix::fix_on_linux;
use crate::core::tools::anim_scale::change_anim_scale;
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
    match &main_work() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) if e.eprintln() != () => unreachable!(),
        Err(e) if let Some(ErrorCode(code, _)) = &e.downcast_ref::<ErrorCode>() => *code,
        Err(e) if e.is::<UserCancelled>() => ExitCode::SUCCESS,
        _ => ExitCode::FAILURE,
    }
}

fn main_work() -> Rslt<()> {
    if let Ok(true) = env::var("LANG").map(|lang| lang.starts_with("ru")) {
        Language::set_language(Language::Ru);
    }
    let mut config = Config::read();
    config.resolve_sdk();
    config.write().soft_unwrap();
    config.update_adb_path();
    let mut args = args().collect::<Vec<String>>();
    let mode = start_name(args.get(0));
    if !mode.is_unknown() {
        args.remove(0);
    }
    return if args.is_empty() && mode.is_adb_ext() {
        INPUT_OR_EXIT.println();
        let mut input = CmdEditor::new()?;
        let success = Rc::new(RefCell::new(None));
        let helper = CmdHelper::from(SUGGESTIONS, success.clone());
        input.set_helper(Some(helper));
        let history_path = history_path();
        if history_path.exists() {
            input.load_history(&history_path)?;
        }
        looper_work(&mut input, &mut config, success);
        input.save_history(&history_path)?;
        Ok(())
    } else {
        work(mode, args, &mut config)
    }
}

fn looper_work(input: &mut CmdEditor, config: &mut Config, success: CmdHighlight) {
    let mut rslt: Option<Rslt<()>> = None;
    loop {
        let previous = &rslt;
        let status = match previous {
            None => string(""),
            Some(Ok(_)) => string("✔ "),
            Some(Err(_)) => string("✘ "),
        };
        let status_range = 0..status.as_bytes().len();
        *success.borrow_mut() = previous.as_ref()
            .map(|rslt| (rslt.is_ok(), status_range));
        let prompt = format!("{status}{ADB_EXT}> ");
        match input.readline(&prompt) {
            Ok(line) => {
                let trimmed = line.trim();
                match trimmed {
                    "" => {
                        rslt = None;
                        continue
                    },
                    CLEAR => {
                        rslt = None;
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
                    Err(e) => e.eprintln(),
                    Ok(args) => rslt = Some(work(StartMode::AdbExt, args, config)),
                };
            }
            Err(ReadlineError::Interrupted) => { // Ctrl-C
                rslt = None;
                continue
            },
            Err(ReadlineError::Eof) => break, // Ctrl-D
            Err(e) => {
                e.eprintln();
                break;
            }
        }
        match &rslt {
            Some(Err(e)) if e.is::<UserCancelled>() => rslt = Some(Ok(())),
            Some(Err(e)) => e.eprintln(),
            _ => (),
        }
    }
}

fn work(mode: StartMode, args: Vec<String>, config: &mut Config) -> Rslt<()> {
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
        VERSION if !mode.is_adb() => print_version(),
        HELP if !mode.is_adb() => get_help(None).println(),
        "shit" => "💩".println(),
        _ => return run_adb(AdbArgs::spawn(args.as_slice()))?.to_rslt(),
    };
    return Ok(())
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
