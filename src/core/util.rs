use std::fs::create_dir_all;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::{Command, exit};
use crate::core::config::Config;
use crate::core::r#const::ERROR_CODE;


pub fn print_the_fuck_out() {
    io::stdout().flush().unwrap();
}

pub fn home_dir() -> String {
    #[allow(deprecated)] // todo replace with a crate
    std::env::home_dir().unwrap().to_str().unwrap().to_string()
}

pub fn gen_home_path(subpath: Option<&str>) -> String {
    let mut path = home_dir();
    if let Some(subpath) = subpath {
        if !subpath.starts_with('/') {
            path.push('/');
        }
        path.push_str(subpath);
    } else {
        path.push('/');
    }
    return path;
}

pub fn ensure_parent_exists(path: &String) {
    let parent = Path::new(&path).parent().unwrap();
    create_dir_all(parent).unwrap();
}

pub fn try_run_hook_and_exit(hook: Option<String>, cmd: String, arg: String) {
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
        let path = config.environment.sdk.unwrap_or("null".to_string());
        println!("{path}");
    }
}
