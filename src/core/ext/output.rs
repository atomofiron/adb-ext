use std::process::Output;

pub trait OutputExt {
    fn convert(self) -> crate::core::utils::output::Output;
}

impl OutputExt for Output {
    fn convert(self) -> crate::core::utils::output::Output {
        crate::core::utils::output::Output::from(self)
    }
}
