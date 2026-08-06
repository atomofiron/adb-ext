use crate::core::ext::Rslt;
use crate::core::ext::option::OptionExt;
use crate::core::ext::path_buf::PathBufExt;
use crate::core::ext::print::PrintExt;
use crate::core::ext::result::ResultExt;
use crate::core::ext::str::StrExt;
use crate::core::utils::destination::Destination;
use crate::core::utils::values::{ADB, ADB_EXT, BUILD_TOOLS, PLATFORM_TOOLS};
use crate::core::utils::string;
use crate::core::utils::strings::NO_CONFIG_PARENT;
use crate::core::utils::system::{adb_name, config_path, default_sdk_dir, make_executable};
use itertools::Itertools;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

pub static mut ADB_PATH: Option<String> = None;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_hook")]
    hook: Option<String>,
    #[serde(default)]
    pub environment: Environment,
    #[serde(default)]
    pub screenshots: Screenshots,
    #[serde(default)]
    pub screencasts: Screencasts,
}
#[derive(Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Environment {
    pub sdk: Option<String>,
    #[serde(rename = "build-tools")]
    build_tools: Option<String>,
    #[serde(rename = "platform-tools")]
    platform_tools: Option<String>,
}
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Screenshots {
    pub name: String,
    pub sources: Vec<String>,
    pub destination: String,
    hook: Option<String>,
}
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Screencasts {
    pub name: String,
    pub sources: Vec<String>,
    pub destination: String,
    hook: Option<String>,
    pub show_taps: bool,
    pub args: String,
}

fn default_hook() -> Option<String> { Some(string("~/Android/hook")) }

impl Default for Config {
    fn default() -> Self {
        Config {
            hook: default_hook(),
            environment: Environment::default(),
            screenshots: Screenshots::default(),
            screencasts: Screencasts::default(),
        }
    }
}

impl Default for Screenshots {
    fn default() -> Self {
        Screenshots {
            name: string("Screenshot_%Y%m%d-%H%M%S.png"),
            sources: vec![
                string("/sdcard/Pictures/Screenshots"),
                string("/sdcard/DCIM/Screenshots"),
            ],
            destination: string("~/Android/Screenshots"),
            hook: Some(string("~/Android/Screenshots/hook")),
        }
    }
}
impl Default for Screencasts {
    fn default() -> Self {
        Screencasts {
            name: string("Screencast_%Y%m%d-%H%M%S.mp4"),
            sources: vec![
                string("/sdcard/Pictures/Screenshots"),
                string("/sdcard/DCIM/Screen recordings"),
                string("/sdcard/Movies"),
            ],
            destination: string("~/Android/Screencasts"),
            hook: Some(string("~/Android/Screencasts/hook")),
            show_taps: true,
            args: string("--bit-rate 5M"),
        }
    }
}

impl Config {

    pub fn read() -> Config {
        File::open(&config_path())
            .ok()
            .and_then(|file| serde_yaml::from_reader(file).ok())
            .unwrap_or_default()
    }

    pub fn write(&self) -> Rslt<()> {
        let config_path = config_path();
        if !config_path.exists() {
            match config_path.parent() {
                None => NO_CONFIG_PARENT.eprintln(),
                Some(parent) => fs::create_dir_all(parent)?,
            }
        }
        let file = File::create(&config_path)?;
        return serde_yaml::to_writer(file, self)
            .boxed()
    }

    pub fn update_adb_path(&self) {
        unsafe {
            ADB_PATH = self.platform_tools()
                .map(|it| it.join(ADB))
                .take_some_if(|it| it.is_file())
                .or_else(|| {
                    let paths = which::which_all(&adb_name())
                        .map(|it| it.collect::<Vec<_>>())
                        .unwrap_or(vec![]);
                    paths.iter()
                        .find_or_first(|it| {
                            match () {
                                _ if !it.is_file() => false,
                                _ if !it.is_symlink() => true,
                                _ if let Ok(path) = it.read_link() => path != ADB_EXT.path(),
                                _ => false,
                            }
                        })
                        .cloned()
                }).map(|p| {
                p.to_string()
            });
        }
    }

    pub fn resolve_sdk(&mut self) {
        self.environment.sdk = self.environment.sdk
            .clone()
            .or_else_try(|| {
                let default = default_sdk_dir();
                match default.is_dir() {
                    true => Path::to_str(&default.replace_home_with_tilda())
                        .map(String::from),
                    false => None,
                }
            });
    }

    pub fn get_adb_path() -> Option<String> {
        return unsafe { Option::clone(&*&raw const ADB_PATH) }
    }

    pub fn build_tools(&self) -> Option<PathBuf> {
        existing_or_none(
            dir_checker,
            self.environment.build_tools.clone().map(|it| it.dst()),
            self.environment.sdk.clone().map(|it| it.dst().join(BUILD_TOOLS)),
        )
    }

    pub fn platform_tools(&self) -> Option<PathBuf> {
        existing_or_none(
            dir_checker,
            self.environment.platform_tools.clone().map(|it| it.dst()),
            self.environment.sdk.clone().map(|it| it.dst().join(PLATFORM_TOOLS)),
        )
    }

    pub fn screenshot_hook(&self) -> Option<PathBuf> {
        existing_or_none(
            file_checker,
            self.screenshots.hook.clone().map(|it| it.dst()),
            self.hook.clone().map(|it| it.dst()),
        ).and_then(|it| make_executable(it).ok())
    }

    pub fn screencast_hook(&self) -> Option<PathBuf> {
        existing_or_none(
            file_checker,
            self.screencasts.hook.clone().map(|it| it.dst()),
            self.hook.clone().map(|it| it.dst()),
        ).and_then(|it| make_executable(it).ok())
    }
}

fn existing_or_none<F>(checker: F, first: Option<PathBuf>, second: Option<PathBuf>) -> Option<PathBuf> where F: Fn(&PathBuf) -> bool {
    first.clone()
        .take_some_if(&checker)
        .or(second)
        .take_some_if(&checker)
}

fn dir_checker(path: &PathBuf) -> bool {
    fs::metadata(path).map(|it| it.is_dir()).unwrap_or(false)
}

fn file_checker(path: &PathBuf) -> bool {
    fs::metadata(path).map(|it| it.is_file()).unwrap_or(false)
}
