use std::process::{exit, Output};

pub trait ShortUnwrap<T> {
    fn short_unwrap(self) -> T;
}

impl <T> ShortUnwrap<T> for Result<T, String> {
    fn short_unwrap(self) -> T {
        match self {
            Ok(value) => value,
            Err(message) => {
                println!("{message}");
                exit(1);
            },
        }
    }
}

pub trait OutputString {
    fn stdout(&self) -> String;
    fn stderr(&self) -> String;
}

impl OutputString for Output {
    fn stdout(&self) -> String {
        self.stdout.clone().trim()
    }
    fn stderr(&self) -> String {
        self.stderr.clone().trim()
    }
}

trait Trim {
    fn trim(self) -> String;
}

impl Trim for Vec<u8> {
    fn trim(self) -> String {
        String::from_utf8(self).unwrap().trim().to_string()
    }
}

pub trait ResultToOption<T> {
    fn to_option(self) -> Option<T>;
}

impl <T,E> ResultToOption<T> for Result<T,E> {
    fn to_option(self) -> Option<T> {
        self.map_or(None, |it| Some(it))
    }
}

pub trait Split {
    fn split_to_vec(&self, pat: char) -> Vec<String>;
    fn splitn_to_vec(&self, n: usize, pat: char) -> Vec<String>;
}

impl Split for str {
    fn split_to_vec(&self, pat: char) -> Vec<String> {
        str::split(self, pat)
            .map(String::from)
            .collect::<Vec<String>>()
    }
    fn splitn_to_vec(&self, n: usize, pat: char) -> Vec<String> {
        str::splitn(self, n, pat)
            .map(String::from)
            .collect::<Vec<String>>()
    }
}
