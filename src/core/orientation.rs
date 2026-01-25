use crate::core::adb_command::AdbArgs;
use crate::core::ext::OutputExt;
use crate::core::r#const::SHELL;
use crate::core::selector::{resolve_device, run_adb_with};
use std::fmt::{Display, Formatter};
use std::process::ExitCode;

const ACCELEROMETER: &str = "settings put system accelerometer_rotation";
const LOCKED: u8 = 0;
const AUTO: u8 = 1;
const USER: &str = "settings put system user_rotation";
const PORTRAIT: u8 = 0;
const LANDSCAPE: u8 = 1;
const FLIPPED: u8 = 2;

pub enum Orientation {
    Accelerometer(bool),
    Portrait(bool),
    Landscape(bool),
}

impl Orientation {
    pub fn accelerometer(enabled: bool) -> Orientation {
        Orientation::Accelerometer(enabled)
    }
    pub fn portrait(flipped: bool) -> Orientation {
        Orientation::Portrait(flipped)
    }
    pub fn landscape(flipped: bool) -> Orientation {
        Orientation::Landscape(flipped)
    }
    pub fn command(&self) -> &str {
        match self {
            Orientation::Accelerometer(_) => ACCELEROMETER,
            Orientation::Portrait(_) => USER,
            Orientation::Landscape(_) => USER,
        }
    }
    pub fn code(&self) -> u8 {
        match self {
            Orientation::Accelerometer(enabled) => if *enabled { AUTO } else { LOCKED },
            Orientation::Portrait(flipped) => PORTRAIT + if *flipped { FLIPPED } else { 0 },
            Orientation::Landscape(flipped) => LANDSCAPE + if *flipped { FLIPPED } else { 0 },
        }
    }
}

impl Display for Orientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.command(), self.code())
    }
}

pub fn orientation(orientation: Orientation) -> ExitCode {
    let command = match orientation {
        Orientation::Accelerometer(_) => format!("{orientation}"),
        _ => format!("{} && {orientation}", Orientation::accelerometer(false)),
    };
    let args = &[SHELL, command.as_str()];
    let device = match resolve_device() {
        Ok(device) => device,
        Err(code) => return code,
    };
    let output = run_adb_with(&device, AdbArgs::run(args));

    if !output.status.success() {
        output.print_err();
    }
    return output.exit_code()
}
