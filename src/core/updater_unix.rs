use crate::core::ext::{OutputExt, PrintExt, Rslt};
use std::path::PathBuf;
use std::process::{Command, ExitCode};
use std::{fs::File, io};

const SCRIPT_URL: &str = "https://github.com/atomofiron/adb-ext/raw/main/stuff/install.sh";
const SCRIPT_NAME: &str = "install-adb-ext.sh";
const SHELL: &str = "sh";

pub fn update_unix() -> ExitCode {
    let path = std::env::temp_dir().join(SCRIPT_NAME);
    match download(SCRIPT_URL, &path) {
        Ok(_) => (),
        Err(e) => {
            e.eprintln();
            return ExitCode::FAILURE;
        }
    };
    return Command::new(SHELL)
        .arg(SCRIPT_NAME)
        .spawn().unwrap()
        .wait_with_output().unwrap()
        .exit_code()
}

pub fn download(url: &str, dst: &PathBuf) -> Rslt<()> {
    let res = ureq::get(url).call()?;

    let mut out = File::create(dst)?;
    let (_, body) = res.into_parts();
    let mut reader = body.into_reader();

    io::copy(&mut reader, &mut out)?;
    Ok(())
}

