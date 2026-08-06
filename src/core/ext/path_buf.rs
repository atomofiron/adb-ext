use std::path::{Path, PathBuf};
use crate::core::utils::r#const::{HOME, NULL};

pub trait PathBufExt {
    fn to_string(&self) -> String;
    fn to_str(&self) -> &str;
    fn is_null_or_empty(&self) -> bool;
    fn replace_home_with_tilda(self) -> Self;
}

impl PathBufExt for PathBuf {

    fn to_string(&self) -> String {
        self.to_string_lossy().to_string()
    }

    fn to_str(&self) -> &str {
        Path::to_str(&self).unwrap()
    }

    fn is_null_or_empty(&self) -> bool {
        match Path::to_str(self) {
            None => false, // it's not empty
            Some(s) => s.is_empty() || s == NULL
        }
    }

    fn replace_home_with_tilda(self) -> Self {
        match dirs::home_dir() {
            None => return self,
            Some(home) => {
                self.strip_prefix(home) // check starts_with inside
                    .ok()
                    .map(|it| Path::new(HOME).join(it))
                    .unwrap_or(self)
            },
        }
    }
}
