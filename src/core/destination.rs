use std::fs;
use chrono::Local;
use crate::core::ext::StrExt;
use crate::core::util::gen_home_path;


pub trait Destination {
    fn with_dir(self, default_sub_path: &str) -> String;
    fn with_file(self, default_template: &str) -> String;
}

impl Destination for String {

    fn with_dir(self, default_sub_path: &str) -> String {
        match () {
            _ if self.starts_with('/') => self,
            _ if self.starts_with("./") => self,
            _ if self.starts_with('~') => gen_home_path(Some(&self[1..])),
            _ => gen_home_path(Some(format!("{default_sub_path}{self}").as_str())),
        }
    }

    fn with_file(mut self, default_name_template: &str) -> String {
        let dot = default_name_template.last_index_of('.').unwrap();
        let ext = &default_name_template.clone()[dot..];
        match () {
            _ if self.ends_with('/') => (),
            _ if fs::metadata(self.clone()).map_or(false, |it| it.is_dir()) => self.push('/'),
            _ if self.to_lowercase().ends_with(ext) => return self,
            _  => return format!("{self}{ext}"),
        };
        format!("{self}{}", Local::now().format(default_name_template))
    }
}