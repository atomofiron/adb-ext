
pub const LSS: &str = "lss";
pub const LSC: &str = "lsc";
pub const MSS: &str = "mss";
pub const SHOT: &str = "shot";
pub const MSC: &str = "msc";
pub const REC: &str = "rec";
pub const RECORD: &str = "record";
pub const RUN: &str = "run";
pub const STEAL: &str = "steal";
pub const PORT: &str = "port";
pub const LAND: &str = "land";
pub const FPORT: &str = "fport";
pub const FLAND: &str = "fland";
pub const ACCEL: &str = "accel";
pub const NOACCEL: &str = "noaccel";
pub const BOUNDS: &str = "bounds";
pub const TAPS: &str = "taps";
pub const POINTER: &str = "pointer";

const DEVICES: &str = "devices";
pub const SHELL: &str = "shell";
pub const PULL: &str = "pull";
const PUSH: &str = "push";
pub const INSTALL: &str = "install";

pub const CLEAR: &str = "clear";
pub const EXIT: &str = "exit";
pub const QUIT: &str = "quit";
pub const DEPLOY: &str = "deploy";
pub const UPDATE: &str = "update";
pub const SDK: &str = "sdk";
pub const FIX: &str = "fix";

pub const HELP: &[&str] = &["lss [count]", "mss|shot [destination]", "lsc [count]", "msc|rec|record [destination]", "bounds", "taps", "pointer", "[f]port|[f]land|[no]accel", "sdk", "adb run app.apk", "adb steal app.package.name", "adb-ext update"];
pub const SUGGESTIONS: &[&str] = &[LSS, LSC, MSS, SHOT, MSC, REC, RECORD, RUN, STEAL, PORT, LAND, FPORT, FLAND, ACCEL, NOACCEL, BOUNDS, TAPS, POINTER, DEVICES, SHELL, PULL, PUSH, INSTALL, CLEAR, EXIT, QUIT];

pub const ADB: &str = "adb";
pub const PLATFORM_TOOLS: &str = "platform-tools";

pub const ERROR_CODE: i32 = 1;
pub const NULL: &str = "null";

pub const ON: &str = "1";
pub const OFF: &str = "0";