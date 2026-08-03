use std::error::Error;
use std::fmt::Display;
use crate::core::ext::print::PrintExt;

pub trait ResultExt<R, E> {
    fn string_err(self) -> Result<R, String>;
    fn soft_unwrap(self) -> Option<R>;
    fn boxed(self) -> Result<R, Box<dyn Error>> where E: Error + Send + Sync + 'static;
}

impl<R, E> ResultExt<R, E> for Result<R, E> where E: Display {

    fn string_err(self) -> Result<R, String> {
        self.map_err(|e| e.to_string())
    }

    fn soft_unwrap(self) -> Option<R> {
        if let Err(e) = &self {
            e.eprintln();
        }
        self.ok()
    }

    fn boxed(self) -> Result<R, Box<dyn Error>> where E: Error + Send + Sync + 'static {
        self.map_err(Into::into)
    }
}
