extern crate dirs;
#[cfg(windows)]
use crate::core::ext::PathBufExt;
#[cfg(windows)]
use crate::core::ext::StrExt;
use crate::core::r#const::ADB;
#[cfg(unix)]
use crate::core::util::string;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::{fs, io};

const ADB_EXT: &str = "adb-ext";
#[cfg(windows)]
pub const DOT_EXE: &str = ".exe";
#[cfg(windows)]
pub const PROGRAMS: &str = "Programs";
#[cfg(windows)]
pub const PATH: &str = "PATH";

#[cfg(unix)]
pub fn kill(id: u32) {
    let pid = nix::unistd::Pid::from_raw(id as nix::libc::pid_t);
    nix::sys::signal::kill(pid, nix::sys::signal::Signal::SIGINT).unwrap();
}

#[cfg(windows)]
pub fn kill(id: u32) {
    use windows_sys::Win32::System::Console::{
        GenerateConsoleCtrlEvent, SetConsoleCtrlHandler, CTRL_BREAK_EVENT,
    };
    unsafe {
        if SetConsoleCtrlHandler(None, 1) == 0 { // don't kill myself
            panic!("SetConsoleCtrlHandler(add) failed: {}", std::io::Error::last_os_error());
        }
        if GenerateConsoleCtrlEvent(CTRL_BREAK_EVENT, id) == 0 {
            let _ = SetConsoleCtrlHandler(None, 0); // return handler back
            panic!("GenerateConsoleCtrlEvent failed: {}", std::io::Error::last_os_error());
        }
        if SetConsoleCtrlHandler(None, 0) == 0 {
            panic!("SetConsoleCtrlHandler(remove) failed: {}", std::io::Error::last_os_error());
        }
    }
}

#[cfg(unix)]
pub fn make_executable(path: PathBuf) -> io::Result<PathBuf> {
    let mut perms = fs::metadata(&path)?.permissions();
    let mode = perms.mode();
    if mode & 0o111 == 0 {
        perms.set_mode(mode | 0o100);
        fs::set_permissions(&path, perms)?;
    }
    return Ok(path);
}

#[cfg(windows)]
pub fn make_executable(path: PathBuf) -> io::Result<PathBuf> {
    let mut perms = fs::metadata(&path)?.permissions();
    if perms.readonly() {
        perms.set_readonly(false);
        fs::set_permissions(&path, perms)?;
    }
    return Ok(path)
}

#[cfg(unix)]
pub fn bin_dir() -> PathBuf {
    home_dir()
        .join(".local")
        .join("bin")
}

#[cfg(windows)]
pub fn bin_dir() -> PathBuf {
    dirs::data_local_dir()
        .expect("no LocalAppData")
        .join(PROGRAMS)
        .join("adb-ext")
}

pub fn bin_name() -> String {
    #[cfg(unix)]
    return string(ADB_EXT);
    #[cfg(windows)]
    return exe_name(ADB_EXT)
}

pub fn adb_name() -> String {
    #[cfg(unix)]
    return ADB.to_string();
    #[cfg(windows)]
    return exe_name(ADB)
}

#[cfg(windows)]
pub fn exe_name(name: &str) -> String {
    format!("{name}{DOT_EXE}")
}

#[cfg(unix)]
pub fn bin_path() -> PathBuf {
    bin_dir().join(ADB_EXT)
}

#[cfg(windows)]
pub fn bin_path() -> PathBuf {
    bin_dir().join(bin_name())
}

#[cfg(unix)]
pub fn config_path() -> PathBuf {
    dirs::config_dir()
        .expect("no ~/.config")
        .join("adb-ext")
        .join("config.yaml")
}

#[cfg(windows)]
pub fn config_path() -> PathBuf {
    dirs::config_dir()
        .expect("no AppData/Roaming")
        .join("adb-ext")
        .join("config.yaml")
}

#[cfg(unix)]
pub fn env_path() -> PathBuf {
    home_dir()
        .join(".local")
        .join("env")
}

pub fn home_dir() -> PathBuf {
    dirs::home_dir().expect("no home dir")
}

#[cfg(unix)]
pub fn remove_link(link: &str) -> io::Result<()> {
    fs::remove_file(link)
}

#[cfg(windows)]
pub fn remove_link(link: &str) -> io::Result<()> {
    fs::remove_file(exe_name(link))
}

#[cfg(unix)]
pub fn make_link(link: &str) -> io::Result<()> {
    std::os::unix::fs::symlink(bin_name(), link)
}

#[cfg(windows)]
pub fn make_link(link: &str) -> io::Result<()> {
    fs::hard_link(bin_name(), exe_name(link))
}

#[cfg(windows)]
pub fn env_adb_ext_path() -> String {
    "%LOCALAPPDATA%".path()
        .join(PROGRAMS)
        .join(ADB_EXT)
        .to_string()
}

