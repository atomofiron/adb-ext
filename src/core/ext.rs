use std::process::{exit, Output};


const MORE_THAN_ONE: &str = "adb: more than one device/emulator";
const NO_TARGETS: &str = "adb: no devices/emulators found";

pub trait ShortUnwrap<T> {
    fn short_unwrap(self) -> T;
}

impl<T> ShortUnwrap<T> for Result<T, String> {
    fn short_unwrap(self) -> T {
        match self {
            Ok(value) => value,
            Err(message) => {
                println!("{message}");
                exit(1);
            }
        }
    }
}

pub trait OutputExt {
    fn code(&self) -> i32;
    fn stdout(&self) -> String;
    fn stderr(&self) -> String;
    fn is_more_than_one(&self) -> bool;
    fn print(&self);
}

impl OutputExt for Output {
    fn code(&self) -> i32 {
        self.status.code().unwrap_or(1)
    }
    fn stdout(&self) -> String {
        self.stdout.clone().trim()
    }
    fn stderr(&self) -> String {
        self.stderr.clone().trim()
    }
    fn is_more_than_one(&self) -> bool {
        !self.status.success() && self.stderr() == MORE_THAN_ONE
    }
    fn print(&self) {
        let stdout = self.stdout();
        if !stdout.is_empty() {
            println!("{stdout}");
        }
        let stderr = self.stderr();
        if !stderr.is_empty() {
            println!("{stderr}");
        }
    }
}

pub fn print_no_one() {
    println!("{NO_TARGETS}");
}

trait Trim {
    fn trim(self) -> String;
}

impl Trim for Vec<u8> {
    fn trim(mut self) -> String {
        let fixed = self.iter()
            .map(|&it| it as u16)
            .collect::<Vec<u16>>();
        String::from_utf16_lossy(&fixed).trim().to_string()
    }
}

pub trait ResultToOption<T> {
    fn to_option(self) -> Option<T>;
}

impl<T, E> ResultToOption<T> for Result<T, E> {
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

pub trait StringVec {
    fn to_string_vec(&self) -> Vec<String>;
}

impl StringVec for Vec<&str> {
    fn to_string_vec(&self) -> Vec<String> {
        self.iter().map(ToString::to_string).collect()
    }
}

pub trait VecExt {
    fn last_index(&self) -> usize;
}

impl<T> VecExt for Vec<T> {
    fn last_index(&self) -> usize {
        self.len() - 1
    }
}

pub trait StrExt {
    fn contains_upper(&self) -> bool;
}

impl StrExt for str {
    fn contains_upper(&self) -> bool {
        // A-Z
        self.chars().any(|it| (65..=90u8).contains(&(it as u8)))
    }
}
