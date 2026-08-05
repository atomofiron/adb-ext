use std::error::Error;
use std::fmt;
use std::process::ExitCode;

#[derive(Debug, Clone)]
pub struct ErrorCode(
    pub ExitCode,
    pub String,
);

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.1.is_empty() {
            true => Ok(()),
            false => write!(f, "{:?}: {}", self.0, self.1),
        }
    }
}

impl Error for ErrorCode {}
