use std::{env, fs};
use std::io::Write;
use itertools::Itertools;
use crate::core::strings::{INSTALLATION, UPDATE};
use crate::core::util::home_dir;


const ENV_VERSION: &str = "4";
const BOLD: &str = "\x1b[1m";
const CLEAR: &str = "\x1b[0m";
const EXAMPLES: &[&str] = &["lss [count]", "mss|shot [destination]", "lsc [count]", "msc|rec|record [destination]", "adb run app.apk", "adb steal app.package.name", "adb update"];


pub fn update() {

}

pub fn deploy() {
    let home = home_dir();
    let local_bin = ".local/bin";
    let env_path = "$HOME/.local/env";
    let env_file = format!("{home}/.local/env");
    let bin_dir = format!("{home}/{local_bin}");
    let adb_ext = "adb-ext";
    let adb_ext_path = format!("{bin_dir}/{adb_ext}2");
    let mut action = INSTALLATION.value();
    for name in ["green-pain", "adb-ext"] {
        if fs::metadata(format!("{bin_dir}/{name}")).is_ok() {
            action = UPDATE.value();
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
    println!("{action} succeed, run {BOLD}{}{CLEAR}", EXAMPLES.iter().join(&sep));
    if !auto_configure || current_env_version != ENV_VERSION {
        println!("... however, first of all to configure your current shell, run:");
        println!("{BOLD}source {env_path}{CLEAR}");
    }
}
