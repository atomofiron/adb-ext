use std::process::{ExitCode, ExitStatus};
use crate::core::util::r#const::ERROR_CODE;

pub trait ExitStatusExt {
    fn exit_code(&self) -> ExitCode;
}

impl ExitStatusExt for ExitStatus {
    fn exit_code(&self) -> ExitCode {
        let code = self.code().unwrap_or(ERROR_CODE) & 255;
        ExitCode::from(code as u8)
    }
}
