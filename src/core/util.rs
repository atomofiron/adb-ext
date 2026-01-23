use std::fs::create_dir_all;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, exit};
use chrono::Local;
use crate::core::config::Config;
use crate::core::r#const::{ERROR_CODE, NULL};

pub fn string(value: &str) -> String {
    String::from(value)
}

pub fn null() -> String {
    string(NULL)
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

pub fn set_sdk(path: Option<String>, mut config: Config) {
    if let Some(_) = path {
        config.environment.sdk = path;
        config.write();
    } else {
        let path = config.environment.sdk.unwrap_or(null());
        println!("{path}");
    }
}

pub fn format_file_name(name: &String) -> String {
    Local::now().format(name).to_string()
}
