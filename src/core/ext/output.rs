use std::process::Output;

pub trait OutputExt {
    fn convert(self) -> crate::core::output::Output;
}

impl OutputExt for Output {
    fn convert(self) -> crate::core::output::Output {
        crate::core::output::Output::from(self)
    }
}
