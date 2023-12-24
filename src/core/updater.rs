use std::{env, fs};
use std::io::Write;
use std::process::{Command, exit};
use itertools::Itertools;
use crate::core::strings::{HOWEVER_CONFIGURE, INSTALLATION_SUCCEED, UPDATE_SUCCEED};
use crate::core::util::home_dir;


const SCRIPT_URL: &str = "https://github.com/atomofiron/adb-ext/raw/main/stuff/install.sh";
const SCRIPT_NAME: &str = "install-adb-ext.sh";
const ENV_VERSION: &str = "4";
const BOLD: &str = "\x1b[1m";
const CLEAR: &str = "\x1b[0m";
const EXAMPLES: &[&str] = &["lss [count]", "mss|shot [destination]", "lsc [count]", "msc|rec|record [destination]", "adb run app.apk", "adb steal app.package.name", "adb-ext update"];

pub fn update() {
    let bytes = reqwest::blocking::get(SCRIPT_URL).unwrap()
        .bytes().unwrap();
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(SCRIPT_NAME).unwrap();
    file.write(&bytes).unwrap();
    std::io::stdin().read_line(&mut String::new()).unwrap();
    let code = Command::new("sh")
        .arg(SCRIPT_NAME)
        .spawn().unwrap()
        .wait().unwrap()
        .code().unwrap();
    exit(code);
}

pub fn deploy() {
    let home = home_dir();
    let local_bin = ".local/bin";
    let env_path = "$HOME/.local/env";
    let env_file = format!("{home}/.local/env");
    let bin_dir = format!("{home}/{local_bin}");
    let adb_ext = "adb-ext";
    let adb_ext_path = format!("{bin_dir}/{adb_ext}");
    let mut action = INSTALLATION_SUCCEED.value();
    for name in ["green-pain", adb_ext] {
        if fs::metadata(format!("{bin_dir}/{name}")).is_ok() {
            action = UPDATE_SUCCEED.value();
        }
    }
    let _ = fs::remove_file(format!("{bin_dir}/green-pain"));
    if fs::metadata(&bin_dir).is_err() {
        fs::create_dir_all(&bin_dir).unwrap();
    }
    fs::copy(env::args().collect::<Vec<String>>().first().unwrap().to_string(), adb_ext_path.clone()).unwrap();
    env::set_current_dir(bin_dir.clone()).unwrap();
    for link in ["lss", "mss", "shot", "lsc", "msc", "rec", "record"] {
        let _ = fs::remove_file(link);
        std::os::unix::fs::symlink(adb_ext, link).unwrap();
    }
    let env = format!("
case \":${{PATH}}:\" in
    *:\"$HOME/{local_bin}\":*)
        ;;
    *)
		export PATH=$HOME/{local_bin}:$PATH
        ;;
esac
alias adb=adb-ext
unalias lss 2>/dev/null
unalias lsc 2>/dev/null
unalias mss 2>/dev/null
unalias shot 2>/dev/null
export ADB_EXT_VERSION_CODE={ENV_VERSION}
");
    fs::write(env_file, env).unwrap();
    let current_env_version = env::var("ADB_EXT_VERSION_CODE").unwrap_or("".to_string());
    let mut auto_configure = !current_env_version.is_empty();
    if !auto_configure {
        for startup in [".profile", ".zshrc", ".bashrc", ".config/fish/config.fish"] {
            if let Ok(mut file) = fs::OpenOptions::new()
                .create(false)
                .write(true)
                .append(true)
                .open(format!("{home}/{startup}")) {
                file.write_all(format!("\n. {env_path}\n").as_bytes()).unwrap();
                auto_configure = true;
            };
        }
    }
    let sep = format!("{CLEAR}, {BOLD}");
    println!("{action} {BOLD}{}{CLEAR}", EXAMPLES.iter().join(&sep));
    if !auto_configure || current_env_version != ENV_VERSION {
        HOWEVER_CONFIGURE.println();
        println!("{BOLD}source {env_path}{CLEAR}");
    }
}
