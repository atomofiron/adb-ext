use crate::core::utils::config::Config;
use crate::core::utils::values::ARG_S;
use crate::core::utils::strings::NO_ADB;
use std::process::Command;

#[derive(Clone)]
pub struct AdbArgs {
    pub args: Vec<String>,
    pub interactive: bool,
}

impl AdbArgs {
    pub fn run<S: ToString>(args: &[S]) -> AdbArgs {
        AdbArgs::new(args, false)
    }
    pub fn spawn<S: ToString>(args: &[S]) -> AdbArgs {
        AdbArgs::new(args, true)
    }
    fn new<S: ToString>(args: &[S], interactive: bool) -> AdbArgs {
        let args = args.iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>();
        AdbArgs { args, interactive }
    }
    pub fn to_command(self, device: Option<String>) -> Result<Command, String> {
        let mut adb = match Config::get_adb_path() {
            None => return Err(NO_ADB.value().to_string()),
            Some(path) => Command::new(path),
        };
        if let Some(device) = device {
            adb.arg(ARG_S);
            adb.arg(device);
        }
        adb.args(self.args);
        return Ok(adb)
    }
}
