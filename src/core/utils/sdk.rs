use crate::core::ext::path_buf::PathBufExt;
use crate::core::ext::print::PrintExt;
use crate::core::ext::string::StringExt;
use crate::core::ext::{Rslt, take_if};
use crate::core::utils::config::Config;
use crate::core::utils::destination::Destination;
use crate::core::utils::values::{HOME, NULL};
use crate::core::utils::strings::{DONE, NO_SUCH_DIRECTORY};
use crate::core::utils::system::home_dir;
use std::env::current_dir;

pub fn set_sdk(path: Option<String>, config: &mut Config) -> Rslt<()> {
    let path = path.map(|it| it.dst());
    return match path {
        None => {
            config.environment.sdk
                .as_deref()
                .unwrap_or(NULL)
                .println();
            Ok(())
        }
        Some(path) if !path.is_null_or_empty() && !path.is_dir() => NO_SUCH_DIRECTORY.to_err(),
        Some(mut path) => {
            if !path.is_null_or_empty() && path.is_relative() {
                path = current_dir()?.join(path);
            }
            let path = match home_dir() {
                _ if path.is_null_or_empty() => path.to_string(),
                home if path.starts_with(&home) => path.to_string().replace(&home.to_str(), HOME),
                _ => path.to_string(),
            };
            config.environment.sdk = take_if(path, |it| !it.is_null_or_empty());
            config.write()?;
            DONE.println();
            Ok(())
        }
    }
}
