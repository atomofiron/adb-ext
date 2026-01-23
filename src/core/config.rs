use crate::core::destination::Destination;
use crate::core::ext::{OptionExt, PathBufExt, ResultToOption, StrExt};
use crate::core::r#const::{ADB, PLATFORM_TOOLS};
use crate::core::system::{adb_name, config_path, make_executable};
use itertools::Itertools;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use crate::core::util::string;

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
    pub show_touches: bool,
    pub args: String,
}

// NOTE: replace the '~' with home dir path for the Windows in the future
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
            show_touches: true,
            args: string("--bit-rate 5M"),
        }
    }
}

impl Config {
    pub fn read() -> Config {
        let config_path = config_path();
        let text = fs::read_to_string(&config_path)
            .unwrap_or_default();
        return serde_yaml::from_str::<Config>(&text)
            .unwrap_or_default();
    }

    pub fn write(&self) {
        let config_path = config_path();
        if !config_path.exists() {
            fs::create_dir_all(config_path.parent().unwrap()).unwrap();
            File::create(&config_path).unwrap();
        }
        let config_text = serde_yaml::to_string(self).unwrap();
        fs::write(config_path, config_text).unwrap();
    }

    pub fn update_adb_path(&self) {
        unsafe {
            ADB_PATH = self.platform_tools()
                .map(|it| it.join(ADB))
                .take_some_if(|it| it.is_file())
                .if_none(|| {
                    let adb_name = adb_name();
                    let platform_adb = PLATFORM_TOOLS.path().join(&adb_name);
                    let paths = which::which_all(&adb_name)
                        .map(|it| it.collect::<Vec<_>>())
                        .unwrap_or(vec![]);
                    paths.iter()
                        .find_or_first(|it| it.is_file() && it.ends_with(&platform_adb))
                        .cloned()
                }).map(|p| p.to_string());
        }
    }

    pub fn get_adb_path() -> Option<String> {
        return unsafe { Option::clone(&*&raw const ADB_PATH) }
    }

    pub fn build_tools(&self) -> Option<PathBuf> {
        existing_or_none(
            dir_checker,
            self.environment.build_tools.clone().map(|it| it.dst()),
            self.environment.sdk.clone().map(|it| it.dst().join("build-tools")),
        )
    }

    pub fn platform_tools(&self) -> Option<PathBuf> {
        existing_or_none(
            dir_checker,
            self.environment.platform_tools.clone().map(|it| it.dst()),
            self.environment.sdk.clone().map(|it| it.dst().join("platform-tools")),
        )
    }

    pub fn screenshot_hook(&self) -> Option<PathBuf> {
        existing_or_none(
            file_checker,
            self.screenshots.hook.clone().map(|it| it.dst()),
            self.hook.clone().map(|it| it.dst()),
        ).and_then(|it| make_executable(it).to_option())
    }

    pub fn screencast_hook(&self) -> Option<PathBuf> {
        existing_or_none(
            file_checker,
            self.screencasts.hook.clone().map(|it| it.dst()),
            self.hook.clone().map(|it| it.dst()),
        ).and_then(|it| make_executable(it).to_option())
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
