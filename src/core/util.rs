use crate::core::ext::result::ResultExt;
use crate::core::ext::user_cancelled::UserCancelled;
use crate::core::ext::vec::VecExt;
use crate::core::ext::Rslt;
use crate::core::r#const::{HELP_TEXT, NULL};
use crate::core::strings::{CANCEL, NO_PARENT};
use crate::core::system::bin_name;
use chrono::Local;
use dialoguer::FuzzySelect;
use itertools::Itertools;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::process::Command;

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

pub fn ensure_parent_exists(path: &PathBuf) -> Rslt<()> {
    let parent = path.parent()
        .ok_or_else(|| NO_PARENT.value())?;
    return create_dir_all(parent).boxed()
}

pub fn try_run_hook_and_exit(hook: PathBuf, cmd: String, arg: PathBuf) -> Rslt<()> {
    Command::new(hook).arg(cmd).arg(arg)
        .spawn()?
        .wait_with_output()
        .unit()
        .boxed()
}

pub fn format_file_name(name: &String) -> String {
    Local::now().format(name).to_string()
}

pub fn interactive_select<T, F: Fn(&T, &Vec<T>) -> String>(prompt: &str, mut items: Vec<T>, label: F) -> Rslt<T> {
    let mut labels = items.iter()
        .map(|it| label(it, &items))
        .collect::<Vec<_>>();
    labels.push(CANCEL.value().to_string()); // not everywhere Esc works
    let selection = FuzzySelect::new()
        .with_prompt(prompt)
        .default(0)
        .items(&labels)
        .interact_opt()?;
    let selection = match selection {
        Some(selection) => selection,
        None => return UserCancelled.to_rslt(),
    };
    return VecExt::try_remove(&mut items, selection)
        .ok_or(UserCancelled)
        .boxed()
}
