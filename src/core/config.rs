use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use serde_derive::{Deserialize, Serialize};
use crate::core::destination::Destination;
use crate::core::util::home_dir;
use crate::core::ext::{OptionExt, OutputExt, ResultToOption, StrExt, StringExt};


pub const CONFIG_PATH: &str = "~/.config/adb-ext.yaml";
static mut ADB_PATH: Option<String> = None;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_hook")]
    hook: Option<String>,
    #[serde(default)]
    environment: Environment,
    #[serde(default)]
    pub screenshots: Screenshots,
    #[serde(default)]
    pub screencasts: Screencasts,
}
#[derive(Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Environment {
    sdk: Option<String>,
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
    pub args: String,
}

// NOTE: replace the '~' with home dir path for the Windows in the future
fn default_hook() -> Option<String> { Some("~/Android/hook".to_string()) }

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
            name: "Screenshot_%Y%m%d-%H%M%S.png".to_string(),
            sources: vec![
                "/sdcard/Pictures/Screenshots".to_string(),
                "/sdcard/DCIM/Screenshots".to_string(),
            ],
            destination: "~/Android/Screenshots".to_string(),
            hook: Some("~/Android/Screenshots/hook".to_string()),
        }
    }
}
impl Default for Screencasts {
    fn default() -> Self {
        Screencasts {
            name: "Screencast_%Y%m%d-%H%M%S.mp4".to_string(),
            sources: vec![
                "/sdcard/Pictures/Screenshots".to_string(),
                "/sdcard/DCIM/Screen recordings".to_string(),
                "/sdcard/Movies".to_string(),
            ],
            destination: "~/Android/Screencasts".to_string(),
            hook: Some("~/Android/Screencasts/hook".to_string()),
            args: "--bit-rate 5M".to_string(),
        }
    }
}

impl Config {
    pub fn read() -> Config {
        let config_path = format!("{}{}", home_dir(), &CONFIG_PATH[1..]);
        let config_path = Path::new(&config_path);
        if !config_path.exists() {
            fs::create_dir_all(config_path.parent().unwrap()).unwrap();
            File::create(config_path).unwrap();
        }
        let text = fs::read_to_string(config_path).unwrap_or("".to_string());
        let config = serde_yaml::from_str::<Config>(&text).unwrap_or_else(|_| Config::default());
        let text = serde_yaml::to_string(&config).unwrap();
        fs::write(config_path, text).unwrap();
        return config;
    }

    pub fn update_adb_path(&self) {
        unsafe {
            ADB_PATH = self.platform_tools()
                .map(|it| format!("{}adb", it.with_slash()))
                .take_some_if(|it| it.is_file())
                .if_none(|| {
                    Command::new("/usr/bin/which").arg("adb")
                        .output()
                        .to_option()
                        .map(|it| it.stdout())
                        .take_some_if(|it| it.is_file())
                });
        }
    }

    pub fn get_adb_path() -> Option<String> {
        return unsafe { ADB_PATH.clone() }
    }

    pub fn build_tools(&self) -> Option<String> {
        existing_or_none(
            dir_checker,
            self.environment.build_tools.clone().map(|it| it.dst()),
            self.environment.sdk.clone().map(|it| it.dst()),
        )
    }

    pub fn platform_tools(&self) -> Option<String> {
        existing_or_none(
            dir_checker,
            self.environment.platform_tools.clone().map(|it| it.dst()),
            self.environment.sdk.clone().map(|it| it.dst()),
        )
    }

    pub fn screenshot_hook(&self) -> Option<String> {
        existing_or_none(
            file_checker,
            self.screenshots.hook.clone().map(|it| it.dst()),
            self.hook.clone().map(|it| it.dst()),
        )
    }

    pub fn screencast_hook(&self) -> Option<String> {
        existing_or_none(
            file_checker,
            self.screencasts.hook.clone().map(|it| it.dst()),
            self.hook.clone().map(|it| it.dst()),
        )
    }
}

fn existing_or_none<F>(checker: F, first: Option<String>, second: Option<String>) -> Option<String> where F: Fn(&String) -> bool {
    first.clone()
        .take_some_if(&checker)
        .or(second)
        .take_some_if(&checker)
}

fn dir_checker(path: &String) -> bool {
    fs::metadata(path).map(|it| it.is_dir()).unwrap_or(false)
}

fn file_checker(path: &String) -> bool {
    fs::metadata(path).map(|it| it.is_file()).unwrap_or(false)
}
