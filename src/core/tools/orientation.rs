use crate::core::ext::Rslt;
use crate::core::utils::adb_args::AdbArgs;
use crate::core::utils::values::SHELL;
use crate::core::utils::selector::run_adb;
use std::fmt::{Display, Formatter};

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

pub fn orientation(orientation: Orientation) -> Rslt<()> {
    let command = match orientation {
        Orientation::Accelerometer(_) => format!("{orientation}"),
        _ => format!("{} && {orientation}", Orientation::accelerometer(false)),
    };
    let args = &[SHELL, command.as_str()];
    let output = run_adb(AdbArgs::run(args))?;
    return output.to_rslt()
}
