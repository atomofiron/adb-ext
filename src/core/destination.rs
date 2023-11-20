use std::fs;
use chrono::Local;
use crate::core::util::gen_home_path;


pub trait Destination {
    fn ensure_dir(self, default_sub_path: &str) -> String;
    fn with_file(self, default_template: &str) -> String;
}

impl Destination for String {

    fn ensure_dir(self, default_sub_path: &str) -> String {
        match () {
            _ if self.starts_with("./") => self,
            _ if self.starts_with('~') => gen_home_path(Some(&self[1..])),
            _ if self.contains('/') => self,
            _ if self.is_empty() => gen_home_path(Some(default_sub_path)),
            _ => self,
        }
    }

    fn with_file(mut self, default_template: &str) -> String {
        match () {
            _ if self.ends_with('/') => (),
            _ if fs::metadata(self.clone()).map_or(false, |it| it.is_dir()) => self.push('/'),
            _ if self.to_lowercase().ends_with(".png") => return self,
            _  => return format!("{self}.png"),
        };
        format!("{self}{}", Local::now().format(default_template))
    }
}