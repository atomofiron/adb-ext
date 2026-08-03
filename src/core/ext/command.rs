use std::ffi::OsStr;
use std::process::Command;

pub trait CommandExt {
    fn some_arg<S: AsRef<OsStr>>(&mut self, arg: Option<S>) -> &mut Self;
}

impl CommandExt for Command {

    fn some_arg<S: AsRef<OsStr>>(&mut self, arg: Option<S>) -> &mut Command {
        match arg {
            None => self,
            Some(arg) => self.arg(arg),
        }
    }
}
