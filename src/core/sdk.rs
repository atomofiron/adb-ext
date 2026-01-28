use std::env::current_dir;
use std::process::ExitCode;
use crate::core::config::Config;
use crate::core::destination::Destination;
use crate::core::ext::{PathBufExt, PrintExt, ResultExt};
use crate::core::r#const::{HOME, NULL};
use crate::core::strings::{DONE, NO_SUCH_DIRECTORY};
use crate::core::system::home_dir;

pub fn set_sdk(path: Option<String>, config: &mut Config) -> ExitCode {
    let path = path.map(|it| it.dst());
    return match path {
        None => {
            config.environment.sdk
                .as_deref()
                .unwrap_or(NULL)
                .println();
            ExitCode::SUCCESS
        }
        Some(path) if !path.is_dir() => {
            NO_SUCH_DIRECTORY.eprintln();
            ExitCode::FAILURE
        }
        Some(mut path) => {
            if path.is_relative() {
                path = match current_dir().soft_unwrap() {
                    Some(current) => current.join(path),
                    None => return ExitCode::FAILURE,
                }
            }
            let path = match home_dir() {
                home if path.starts_with(&home) => path.to_string().replace(&home.to_str(), HOME),
                _ => path.to_string(),
            };
            config.environment.sdk = Some(path);
            match config.write() {
                Ok(_) => DONE.println(),
                Err(e) => e.eprintln(),
            }
            ExitCode::SUCCESS
        }
    }
}
