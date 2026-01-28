use crate::core::system::home_dir;
use std::path::PathBuf;
use crate::core::ext::StringExt;
use crate::core::r#const::HOME;
use crate::core::util::string;

pub trait Destination {
    fn dst(&self) -> PathBuf;
    fn dst_with_parent(&self, default_sub_path: &str) -> PathBuf;
}

impl Destination for String {

    fn dst(&self) -> PathBuf { self.dst_with_parent("") }

    fn dst_with_parent(&self, default_parent: &str) -> PathBuf {
        match () {
            _ if self == HOME => home_dir(),
            _ if self == "." ||
                self == ".." ||
                self.starts_with("./") ||
                self.starts_with("../") ||
                self.starts_with('/') => self.path(),
            _ if self.starts_with("~/") => home_dir().join(&self[2..]),
            _ if default_parent.is_empty() => self.path(),
            _ => string(default_parent)
                .dst()
                .join(self),
        }
    }
}