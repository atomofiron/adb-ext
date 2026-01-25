use std::fs::create_dir_all;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, exit};
use chrono::Local;
use itertools::Itertools;
use crate::core::config::Config;
use crate::core::r#const::{ERROR_CODE, NULL};

const EXAMPLES: &[&str] = &["lss [count]", "mss|shot [destination]", "lsc [count]", "msc|rec|record [destination]", "bounds", "taps", "pointer", "[f]port|[f]land|[no]accel", "adb run app.apk", "adb steal app.package.name", "adb-ext update"];

pub fn get_help(separator: Option<&str>) -> String {
    let sep = separator.unwrap_or(", ");
    EXAMPLES.iter().join(sep)
}

pub fn string(value: &str) -> String {
    String::from(value)
}

pub fn null() -> String {
    string(NULL)
}

pub fn println(message: &str) {
    println!("{}", message)
}

pub fn print_the_fuck_out() {
    io::stdout().flush().unwrap();
}

pub fn ensure_parent_exists(path: &PathBuf) {
    let parent = path.parent().unwrap();
    create_dir_all(parent).unwrap();
}

pub fn try_run_hook_and_exit(hook: Option<PathBuf>, cmd: String, arg: PathBuf) {
    if let Some(hook) = hook {
        Command::new(hook).arg(cmd).arg(arg)
            .spawn().unwrap()
            .wait()
            .map_or_else(
                |_| exit(ERROR_CODE),
                |it| exit(it.code().unwrap_or(ERROR_CODE))
            )
    }
}

pub fn set_sdk(path: Option<String>, config: &mut Config) {
    if let Some(_) = path {
        config.environment.sdk = path;
        config.write();
    } else {
        let path = config.environment.sdk
            .as_deref()
            .unwrap_or(NULL);
        println(path);
    }
}

pub fn format_file_name(name: &String) -> String {
    Local::now().format(name).to_string()
}
