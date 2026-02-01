use crate::core::ext::{OutputExt, ResultExt, VecExt};
use crate::core::r#const::{HELP_TEXT, NULL};
use crate::core::strings::CANCEL;
use chrono::Local;
use dialoguer::FuzzySelect;
use itertools::Itertools;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::process::{Command, ExitCode};
use crate::core::system::bin_name;

pub fn get_help(separator: Option<&str>) -> String {
    let sep = separator.unwrap_or(", ");
    HELP_TEXT.iter().join(sep)
}

pub fn print_version() {
    println!("{} v{}", bin_name(), env!("CARGO_PKG_VERSION"))
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

pub fn interactive_select<T, F: Fn(&T, &Vec<T>) -> String>(prompt: &str, mut items: Vec<T>, label: F) -> Result<T, ExitCode> {
    let mut labels = items.iter()
        .map(|it| label(it, &items))
        .collect::<Vec<_>>();
    labels.push(CANCEL.value().to_string()); // not everywhere Esc works
    let selection = FuzzySelect::new()
        .with_prompt(prompt)
        .default(0)
        .items(&labels)
        .interact_opt()
        .soft_unwrap();
    let selection = match selection {
        Some(Some(selection)) => selection,
        Some(None) => return Err(ExitCode::SUCCESS), // cancel
        None => return Err(ExitCode::FAILURE),
    };
    return VecExt::try_remove(&mut items, selection)
        .ok_or_else(|| ExitCode::SUCCESS) // cancel
}
