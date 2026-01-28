use std::fs::create_dir_all;
use std::io;
use std::io::{Error, Write};
use std::path::PathBuf;
use std::process::{Command, ExitCode};
use chrono::Local;
use itertools::Itertools;
use crate::core::config::Config;
use crate::core::ext::{OutputExt, PrintExt};
use crate::core::r#const::{HELP, NULL};
use crate::core::strings::DONE;

pub fn get_help(separator: Option<&str>) -> String {
    let sep = separator.unwrap_or(", ");
    HELP.iter().join(sep)
}

pub fn string(value: &str) -> String {
    String::from(value)
}

pub fn null() -> String {
    string(NULL)
}

pub fn failure<T>() -> Result<T, ExitCode> {
    Err(ExitCode::FAILURE)
}

pub fn println(message: &str) {
    println!("{}", message)
}

pub fn eprintln(message: &str) {
    eprintln!("{}", message)
}

pub fn print_the_fuck_out() {
    io::stdout().flush().unwrap();
}

pub fn ensure_parent_exists(path: &PathBuf) {
    let parent = path.parent().unwrap();
    create_dir_all(parent).unwrap();
}

pub fn try_run_hook_and_exit(hook: PathBuf, cmd: String, arg: PathBuf) -> ExitCode {
    Command::new(hook).arg(cmd).arg(arg)
        .spawn().unwrap()
        .wait_with_output().unwrap()
        .exit_code()
}

pub fn set_sdk(path: Option<String>, config: &mut Config) -> ExitCode {
    if let Some(_) = path {
        config.environment.sdk = path;
        match config.write() {
            Ok(_) => DONE.println(),
            Err(e) => e.eprintln(),
        }
    } else {
        let path = config.environment.sdk
            .as_deref()
            .unwrap_or(NULL);
        println(path);
    }
    return ExitCode::SUCCESS
}

pub fn format_file_name(name: &String) -> String {
    Local::now().format(name).to_string()
}
