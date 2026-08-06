use std::process::Output;

pub trait OutputExt {
    fn convert(self) -> crate::core::util::output::Output;
}

impl OutputExt for Output {
    fn convert(self) -> crate::core::util::output::Output {
        crate::core::util::output::Output::from(self)
    }
}
