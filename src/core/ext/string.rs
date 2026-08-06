use std::path::PathBuf;
use crate::core::util::r#const::NULL;

pub trait StringExt {
    fn contains_ci(&self, other: &String) -> bool;
    fn path(&self) -> PathBuf;
    fn is_null_or_empty(&self) -> bool;
}

impl StringExt for String {

    fn contains_ci(&self, other: &String) -> bool {
        self.to_lowercase().contains(other.to_lowercase().as_str())
    }

    fn path(&self) -> PathBuf {
        PathBuf::from(self)
    }

    fn is_null_or_empty(&self) -> bool {
        self.is_empty() || self == NULL
    }
}
