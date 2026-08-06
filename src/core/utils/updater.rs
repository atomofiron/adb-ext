use crate::core::ext::exit_status::ExitStatusExt;
use crate::core::ext::path_buf::PathBufExt;
#[cfg(unix)]
use crate::core::ext::print::PrintExt;
use crate::core::ext::Rslt;
use crate::core::ext::result::ResultExt;
use crate::core::utils::values::*;
use crate::core::ext::output::OutputExt;
use crate::core::utils::strings::{DONE, NO_ARGS};
use crate::core::utils::strings::{HOWEVER_CONFIGURE, INSTALLATION_SUCCEED, SYMLINK_FAIL, UPDATE_SUCCEED};
use crate::core::utils::system::{bin_dir, bin_path, make_link, remove_link};
use crate::core::utils::system::{bin_name, make_executable};
#[cfg(windows)]
use crate::core::utils::system::{PATH, env_adb_ext_path};
#[cfg(unix)]
use crate::core::utils::system::{env_path, home_dir};
use crate::core::utils::get_help;
#[cfg(unix)]
use crate::core::utils::string;
use indicatif::{ProgressBar, ProgressStyle};
#[cfg(unix)]
use std::io::Write;
use std::path::PathBuf;
#[cfg(target_os = "macos")]
use std::process::Stdio;
#[cfg(windows)]
use crate::core::ext::string::StringExt;
use std::process::{Command, ExitCode};
use std::{env, fs};
use std::{fs::File, io};

#[cfg(unix)]
const ENV_VERSION: &str = "5";
#[cfg(unix)]
const BOLD: &str = "\x1b[1m";
#[cfg(unix)]
const CLEAR: &str = "\x1b[0m";

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
const URL: &str = "https://github.com/atomofiron/adb-ext/releases/latest/download/adb-ext-apple-arm";
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
const URL: &str = "https://github.com/atomofiron/adb-ext/releases/latest/download/adb-ext-apple-x86_64";
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
const URL: &str = "https://github.com/atomofiron/adb-ext/releases/latest/download/adb-ext-linux-x86_64";
#[cfg(windows)]
const URL: &str = "https://github.com/atomofiron/adb-ext/releases/latest/download/adb-ext.exe";

pub fn update() -> Rslt<()> {
    let mut path = env::temp_dir().join(bin_name());
    download_with_progress(URL, &path)?;
    path = make_executable(path)?;
    #[cfg(target_os = "macos")]
    Command::new("xattr")
        .arg("-d")
        .arg("com.apple.quarantine")
        .arg(&path)
        .stderr(Stdio::null())
        .spawn().soft_unwrap()
        .map(|mut child| child.wait());
    let output = Command::new(&path)
        .arg(DEPLOY)
        .spawn()?
        .wait_with_output()?;
    if output.status.exit_code() == ExitCode::SUCCESS {
        fs::remove_file(path).soft_unwrap();
    }
    return output.convert().to_rslt();
}

fn download_with_progress(url: &str, dst: &PathBuf) -> Rslt<()> {
    let res = ureq::get(url).call()?;
    let total = res.body().content_length();
    let bar = match total {
        Some(n) => ProgressBar::new(n),
        None => ProgressBar::no_length(),
    };
    let style = ProgressStyle::with_template("{spinner} {bytes}/{total_bytes} ({bytes_per_sec}) {bar:40} {eta}")?;
    bar.set_style(style);

    let mut out = File::create(dst)?;
    let (_, body) = res.into_parts();
    let mut reader = bar.wrap_read(body.into_reader());

    io::copy(&mut reader, &mut out)?;
    bar.finish_with_message(DONE.value());

    return Ok(())
}

pub fn deploy() -> Rslt<()> {
    let bin_dir = bin_dir();
    let bin_path = bin_path();

    let mut action = INSTALLATION_SUCCEED.value();
    #[cfg(unix)]
    let current = [&bin_dir.join("green-pain"), &bin_path];
    #[cfg(windows)]
    let current = [&bin_path];
    for path in current {
        if fs::metadata(path).is_ok() {
            action = UPDATE_SUCCEED.value();
            // try to fix on the next line: Os { code: 26, kind: ExecutableFileBusy, message: "Text file busy" }
            let _ = fs::remove_file(path);
        }
    }
    if fs::metadata(&bin_dir).is_err() {
        fs::create_dir_all(&bin_dir)?;
    }
    let src = env::args().nth(0)
        .ok_or_else(|| NO_ARGS.value())?;
    fs::copy(src, &bin_path)?;
    env::set_current_dir(&bin_dir)?;
    for link in [ADB, LSS, MSS, SHOT, LSC, MSC, REC, RECORD, BOUNDS, TAPS, POINTER, PORT, LAND, FPORT, FLAND, ACCEL, NO_ACCEL, ANI_SCALE, STEAL, RUN] {
        let _ = remove_link(link);
        make_link(link).unwrap_or_else(|e|
            println!("{SYMLINK_FAIL}{link} ({e})")
        );
    }
    init_env(action)?;
    return Ok(())
}

#[cfg(unix)]
fn init_env(action: &str) -> Rslt<()> {
    let bin_dir = bin_dir().to_string();
    let env = format!("
#!/bin/sh
# adb-ext shell setup
if [[ \":$PATH:\" != *:\"{bin_dir}\":* ]]; then
    export PATH={bin_dir}:$PATH
fi
unalias adb 2>/dev/null
unalias lss 2>/dev/null
unalias lsc 2>/dev/null
unalias mss 2>/dev/null
unalias shot 2>/dev/null
export ADB_EXT_VERSION_CODE={ENV_VERSION}
");
    let env_path = env_path();
    fs::write(&env_path, env)?;
    let current_env_version = env::var("ADB_EXT_VERSION_CODE").unwrap_or(string(""));
    let mut auto_configure = !current_env_version.is_empty();
    if !auto_configure {
        for startup in [".profile", ".zshrc", ".bashrc", ".config/fish/config.fish"] {
            if let Ok(mut file) = fs::OpenOptions::new()
                .create(false)
                .write(true)
                .append(true)
                .open(home_dir().join(startup)) {
                file.write_all(format!("\n. {}\n", env_path.to_string()).as_bytes())?;
                auto_configure = true;
            };
        }
    }
    let sep = format!("{CLEAR}, {BOLD}");
    println!("{action} {BOLD}{}{CLEAR}", get_help(Some(&sep)));
    if !auto_configure || current_env_version != ENV_VERSION {
        HOWEVER_CONFIGURE.println();
        println!("{BOLD}source {}{CLEAR}", env_path.to_string());
    }
    return Ok(())
}

#[cfg(windows)]
fn init_env(action: &str) -> Rslt<()> {
    println!("{action} {}", get_help(Some(", ")));
    if !path_contains(&bin_dir().to_string()) {
        HOWEVER_CONFIGURE.println_formatted(&[&env_adb_ext_path()]);
    }
    return Ok(())
}

#[cfg(windows)]
pub fn path_contains(dir: &str) -> bool {
    let Some(path_os) = env::var_os(PATH) else {
        return false;
    };
    let path = norm(&dir);
    return env::split_paths(&path_os)
        .any(|p| norm(p.to_str()) == path)
}

#[cfg(windows)]
fn norm(path: &str) -> PathBuf {
    let mut path = path.trim()
        .trim_matches('"')
        .to_string()
        .replace('/', "\\");
    while path.ends_with('\\') && path.len() > 3 {
        path.pop();
    }
    return path.to_ascii_lowercase().path()
}
