use crate::core::ext::OutputExt;
use crate::core::r#const::{HELP, NULL};
use chrono::Local;
use itertools::Itertools;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::process::{Command, ExitCode};

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

pub fn format_file_name(name: &String) -> String {
    Local::now().format(name).to_string()
}
