use std::process::Command;
use crate::core::config::Config;
use crate::core::strings::NO_ADB;

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
        let args = args.iter().map(ToString::to_string).collect::<Vec<String>>();
        AdbArgs { args, interactive }
    }
    pub fn command(self) -> Result<Command, String> {
        let mut adb = match Config::get_adb_path() {
            None => return Err(NO_ADB.value().to_string()),
            Some(path) => Command::new(path),
        };
        adb.args(self.args);
        return Ok(adb)
    }
}

impl Clone for AdbArgs {
    fn clone(&self) -> Self {
        AdbArgs {
            args: self.args.clone(),
            interactive: self.interactive,
        }
    }
}
