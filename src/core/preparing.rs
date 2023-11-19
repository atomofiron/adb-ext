use std::fs;
use chrono::Local;
use crate::core::r#const::{SLASH, TILDE};
use crate::core::util::gen_home_path;


pub struct Destination {}

impl Destination {
    pub fn from(path: Option<String>, default_sub_path: &str) -> String {
        path.unwrap_or_else(|| gen_home_path(Some(default_sub_path)))
    }
}

pub trait DestinationExt {
    fn replace_tilde(self) -> String;
    fn with_file(self, default_template: &str) -> String;
}

impl DestinationExt for String {

    fn replace_tilde(self) -> String {
        if self.starts_with(TILDE) { gen_home_path(Some(&self[1..])) } else { self }
    }

    fn with_file(mut self, default_template: &str) -> String {
        match () {
            _ if self.ends_with(SLASH) => (),
            _ if fs::metadata(self.clone()).map_or(false, |it| it.is_dir()) => self.push(SLASH),
            _ => return self,
        };
        format!("{self}{}", Local::now().format(default_template))
    }
}