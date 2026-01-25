use crate::core::ext::PathBufExt;
#[cfg(windows)]
use crate::core::ext::StringExt;
use crate::core::r#const::*;
use crate::core::strings::{HOWEVER_CONFIGURE, INSTALLATION_SUCCEED, SYMLINK_FAIL, UPDATE_SUCCEED};
use crate::core::system::{bin_dir, bin_path, make_link, remove_link};
#[cfg(windows)]
use crate::core::system::{env_adb_ext_path, PATH};
#[cfg(unix)]
use crate::core::system::{env_path, home_dir};
use crate::core::util::get_help;
#[cfg(unix)]
use crate::core::util::string;
use std::io::Write;
#[cfg(windows)]
use std::path::PathBuf;
use std::process::{exit, Command};
use std::{env, fs};

#[cfg(unix)]
const SCRIPT_URL: &str = "https://github.com/atomofiron/adb-ext/raw/main/stuff/install.sh";
#[cfg(windows)]
const SCRIPT_URL: &str = "https://github.com/atomofiron/adb-ext/raw/main/stuff/install.bat";
#[cfg(unix)]
const SCRIPT_NAME: &str = "install-adb-ext.sh";
#[cfg(windows)]
const SCRIPT_NAME: &str = "install-adb-ext.bat";
#[cfg(unix)]
const SCRIPT_ARGS: [&str; 1] = [SCRIPT_NAME];
#[cfg(windows)]
const SCRIPT_ARGS: [&str; 2] = ["/c", SCRIPT_NAME];
#[cfg(unix)]
const ENV_VERSION: &str = "5";
#[cfg(unix)]
const BOLD: &str = "\x1b[1m";
#[cfg(unix)]
const CLEAR: &str = "\x1b[0m";

#[cfg(unix)]
const SHELL: &str = "sh";
#[cfg(windows)]
const SHELL: &str = "cmd";

pub fn update() {
    let bytes = reqwest::blocking::get(SCRIPT_URL).unwrap()
        .bytes().unwrap();
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(SCRIPT_NAME).unwrap();
    file.write(&bytes).unwrap();
    let code = Command::new(SHELL)
        .args(SCRIPT_ARGS)
        .spawn().unwrap()
        .wait().unwrap()
        .code().unwrap();
    exit(code);
}

pub fn deploy() {
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
        fs::create_dir_all(&bin_dir).unwrap();
    }
    let src = env::args().nth(0).unwrap();
    fs::copy(src, &bin_path).unwrap();
    env::set_current_dir(&bin_dir).unwrap();
    for link in [ADB, LSS, MSS, SHOT, LSC, MSC, REC, RECORD, BOUNDS, TAPS, POINTER, PORT, LAND, FPORT, FLAND, ACCEL, NOACCEL] {
        let _ = remove_link(link);
        make_link(link).unwrap_or_else(|e|
            println!("{SYMLINK_FAIL}{link} ({e})")
        );
    }
    init_env(action);
}

#[cfg(unix)]
fn init_env(action: &str) {
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
    fs::write(&env_path, env).unwrap();
    let current_env_version = env::var("ADB_EXT_VERSION_CODE").unwrap_or(string(""));
    let mut auto_configure = !current_env_version.is_empty();
    if !auto_configure {
        for startup in [".profile", ".zshrc", ".bashrc", ".config/fish/config.fish"] {
            if let Ok(mut file) = fs::OpenOptions::new()
                .create(false)
                .write(true)
                .append(true)
                .open(home_dir().join(startup)) {
                file.write_all(format!("\n. {}\n", env_path.to_string()).as_bytes()).unwrap();
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
}

#[cfg(windows)]
fn init_env(action: &str) {
    println!("{action} {}", get_help(Some(", ")));
    if !path_contains(&bin_dir().to_string()) {
        HOWEVER_CONFIGURE.println_formatted(&[&env_adb_ext_path()]);
    }
}

#[cfg(windows)]
pub fn path_contains(dir: &str) -> bool {
    let Some(path_os) = env::var_os(PATH) else {
        return false;
    };
    let path = norm(&dir);
    return env::split_paths(&path_os)
        .any(|p| norm(p.to_str().unwrap()) == path)
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
