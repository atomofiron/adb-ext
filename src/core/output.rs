use crate::core::ext::{ExitStatusExt, PrintExt, Trim};
use std::process::ExitCode;

pub struct Output {
    pub code: ExitCode,
    pub stdout: Vec<u8>,
    stderr: Vec<u8>,
    stdout_string: Option<String>,
    stderr_string: Option<String>,
}

impl Output {

    pub fn from_error(error: String) -> Self {
        Self {
            code: ExitCode::FAILURE,
            stdout: vec![],
            stderr: vec![],
            stdout_string: None,
            stderr_string: Some(error),
        }
    }

    pub fn success(&self) -> bool {
        self.code == ExitCode::SUCCESS
    }

    pub fn failure(&self) -> bool {
        self.code == ExitCode::FAILURE
    }

    pub fn stdout(&mut self) -> String {
        Self::get(&self.stdout, &mut self.stdout_string)
    }

    pub fn stderr(&mut self) -> String {
        Self::get(&self.stderr, &mut self.stderr_string)
    }

    fn get(src: &Vec<u8>, cache: &mut Option<String>) -> String {
        cache.clone().unwrap_or_else(|| {
            let string = src.fix_nbsp_and_trim();
            *cache = Some(string.clone());
            string
        })
    }

    pub fn print_out(&mut self) {
        let stdout = self.stdout();
        if !stdout.is_empty() {
            stdout.println()
        }
    }

    pub fn print_err(&mut self) {
        let stderr = self.stderr();
        if !stderr.is_empty() {
            stderr.eprintln();
        }
    }

    pub fn print_out_and_err(&mut self) {
        self.print_out();
        self.print_err();
    }
}

impl From<ExitCode> for Output {
    fn from(code: ExitCode) -> Self {
        Self {
            code,
            stdout: vec![],
            stderr: vec![],
            stdout_string: None,
            stderr_string: None,
        }
    }
}

impl From<std::process::Output> for Output {
    fn from(value: std::process::Output) -> Self {
        Self {
            code: value.status.exit_code(),
            stdout: value.stdout,
            stderr: value.stderr,
            stdout_string: None,
            stderr_string: None,
        }
    }
}
