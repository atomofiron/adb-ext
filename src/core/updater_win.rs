use crate::core::ext::{OutputExt, PrintExt, ResultExt, Rslt};
use crate::core::r#const::DEPLOY;
use crate::core::strings::DONE;
use crate::core::system::bin_name;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::remove_file;
use std::path::PathBuf;
use std::process::{Command, ExitCode};
use std::{fs::File, io};

const EXE_URL: &str = "https://github.com/atomofiron/adb-ext/releases/latest/download/adb-ext.exe";

pub fn update_win() -> ExitCode {
    let path = std::env::temp_dir().join(bin_name());
    match download_with_progress(EXE_URL, &path) {
        Ok(_) => (),
        Err(e) => {
            e.eprintln();
            return ExitCode::FAILURE;
        }
    };
    let exit_code = Command::new(&path)
        .arg(DEPLOY)
        .spawn().unwrap()
        .wait_with_output().unwrap()
        .exit_code();
    if exit_code == ExitCode::SUCCESS {
        remove_file(path).soft_unwrap();
    }
    return exit_code;
}

fn download_with_progress(url: &str, dst: &PathBuf) -> Rslt<()> {
    let res = ureq::get(url).call()?;
    let total = res.body().content_length();
    let bar = match total {
        Some(n) => ProgressBar::new(n),
        None => ProgressBar::no_length(),
    };
    let style = ProgressStyle::with_template("{spinner} {bytes}/{total_bytes} ({bytes_per_sec}) {bar:40} {eta}")
        .unwrap();
    bar.set_style(style);

    let mut out = File::create(dst)?;
    let (_, body) = res.into_parts();
    let mut reader = bar.wrap_read(body.into_reader());

    io::copy(&mut reader, &mut out)?;
    bar.finish_with_message(DONE.value());

    Ok(())
}

