use std::fs;
use std::fs::File;
use std::path::Path;
use serde_derive::{Deserialize, Serialize};
use crate::core::util::home_dir;


pub const CONFIG_PATH: &str = "~/.config/adb-ext.yaml";

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_hook")]
    pub hook: String,
    #[serde(default)]
    pub environment: Environment,
    #[serde(default)]
    pub screenshots: Screenshots,
    #[serde(default)]
    pub screencasts: Screencasts,
}
#[derive(Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Environment {
    #[serde(rename = "build-tools")]
    pub build_tools: Option<String>,
    #[serde(rename = "platform-tools")]
    pub platform_tools: Option<String>,
}
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Screenshots {
    pub name: String,
    pub sources: Vec<String>,
    pub destination: String,
    pub hook: String,
}
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Screencasts {
    pub name: String,
    pub sources: Vec<String>,
    pub destination: String,
    pub hook: String,
    pub args: String,
}

// NOTE: replace the '~' with home dir path for the Windows in the future
fn default_hook() -> String { "~/Android/hook".to_string() }

impl Default for Screenshots {
    fn default() -> Self {
        Screenshots {
            name: "Screenshot_%Y%m%d-%H%M%S.png".to_string(),
            sources: vec![
                "/sdcard/Pictures/Screenshots".to_string(),
                "/sdcard/DCIM/Screenshots".to_string(),
            ],
            destination: "~/Android/Screenshots".to_string(),
            hook: "~/Android/Screenshots/hook".to_string(),
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
            hook: "~/Android/Screencasts/hook".to_string(),
            args: "--bit-rate 5M".to_string(),
        }
    }
}

pub fn get_config() -> Config {
    let config_path = get_config_path();
    let config_path = Path::new(&config_path);
    if !config_path.exists() {
        fs::create_dir_all(config_path.parent().unwrap()).unwrap();
        File::create(config_path).unwrap();
    }
    let config = config::Config::builder()
        .add_source(config::File::with_name(config_path.to_str().unwrap()))
        .build().unwrap()
        .try_deserialize::<Config>().unwrap();
    let text = serde_yaml::to_string(&config).unwrap();
    fs::write(config_path, text).unwrap();
    return config;
}

fn get_config_path() -> String { format!("{}{}", home_dir(), &CONFIG_PATH[1..]) }
