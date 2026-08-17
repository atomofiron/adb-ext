use crate::core::ext::error_code::ErrorCode;
use crate::core::ext::exit_status::ExitStatusExt;
use crate::core::ext::print::PrintExt;
use crate::core::ext::trim::Trim;
use crate::core::ext::Rslt;
use std::process::ExitCode;

#[derive(Debug)]
pub struct Output {
    pub code: ExitCode,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    stdout_string: Option<String>,
    stderr_string: Option<String>,
}

impl Output {

    pub fn success(&self) -> bool {
        self.code == ExitCode::SUCCESS
    }

    pub fn stdout(&mut self) -> &str {
        Self::get_cached(&self.stdout, &mut self.stdout_string)
    }

    pub fn stderr(&mut self) -> &str {
        Self::get_cached(&self.stderr, &mut self.stderr_string)
    }

    fn get_cached<'a>(src: &Vec<u8>, cache: &'a mut Option<String>) -> &'a str {
        if let Some(string) = cache {
            string
        } else {
            let string = src.fix_nbsp_and_trim();
            *cache = Some(string);
            Self::get_cached(src, cache)
        }
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

    pub fn to_rslt(mut self) -> Rslt<()> {
        match self.code {
            ExitCode::SUCCESS => Ok(()),
            _ => Err(ErrorCode(self.code, self.stderr().to_string()).into()),
        }
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
