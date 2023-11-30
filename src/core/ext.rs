use std::ffi::OsStr;
use std::process::{Command, exit, Output};
use crate::core::r#const::ERROR_CODE;


const MORE_THAN_ONE: &str = "adb: more than one device/emulator";
const NO_TARGETS: &str = "adb: no devices/emulators found";

const NBSP: u8 = 0xA0;
const BF: u8 = 0xBF;
const C2: u8 = 0xC2;

pub trait ShortUnwrap<T> {
    fn short_unwrap(self) -> T;
}

impl<T> ShortUnwrap<T> for Result<T, String> {
    fn short_unwrap(self) -> T {
        match self {
            Ok(value) => value,
            Err(message) => {
                println!("{message}");
                exit(ERROR_CODE);
            }
        }
    }
}

pub trait OutputExt {
    fn code(&self) -> i32;
    fn stdout(&self) -> String;
    fn stderr(&self) -> String;
    fn is_more_than_one(&self) -> bool;
    fn print_out(&self);
    fn print_err(&self);
    fn print_out_and_err(&self);
}

impl OutputExt for Output {
    fn code(&self) -> i32 {
        self.status.code().unwrap_or(ERROR_CODE)
    }
    fn stdout(&self) -> String {
        self.stdout.fix_nbsp_and_trim()
    }
    fn stderr(&self) -> String {
        self.stderr.fix_nbsp_and_trim()
    }
    fn is_more_than_one(&self) -> bool {
        !self.status.success() && self.stderr() == MORE_THAN_ONE
    }
    fn print_out(&self) {
        let stdout = self.stdout();
        if !stdout.is_empty() {
            println!("{stdout}");
        }
    }
    fn print_err(&self) {
        let stderr = self.stderr();
        if !stderr.is_empty() {
            println!("{stderr}");
        }
    }
    fn print_out_and_err(&self) {
        self.print_out();
        self.print_err();
    }
}

pub fn print_no_one() {
    println!("{NO_TARGETS}");
}

trait Trim {
    fn fix_nbsp_and_trim(&self) -> String;
}

impl Trim for Vec<u8> {
    fn fix_nbsp_and_trim(&self) -> String {
        match count_nbsp(&self) {
            0 => String::from_utf8_lossy(self).trim().to_string(),
            count => String::from_utf8_lossy(&fix_nbsp(self, count)).trim().to_string(),
        }
    }
}

fn count_nbsp(bytes: &Vec<u8>) -> usize {
    let mut count = 0;
    let mut prev: u8 = 0;
    for &byte in bytes {
        if byte == NBSP && prev <= BF {
            count += 1;
        }
        prev = byte;
    }
    return count;
}

fn fix_nbsp(bytes: &Vec<u8>, count: usize) -> Vec<u8> {
    let mut utf8 = Vec::with_capacity(bytes.len() + count);
    let mut prev: u8 = 0;
    for &byte in bytes {
        if byte == NBSP && prev <= BF {
            utf8.push(C2);
        }
        utf8.push(byte);
        prev = byte;
    }
    return utf8;
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
    fn last_index(&self) -> usize;
    fn contains_upper(&self) -> bool;
    fn index_of(&self, c: char) -> Option<usize>;
    fn index_of_or(&self, default: usize, c: char) -> usize;
    fn last_index_of(&self, c: char) -> Option<usize>;
    fn last_index_of_or(&self, default: usize, c: char) -> usize;
}

fn inner_index_of(value: &str, c: char, rev: bool) -> Option<usize> {
    let mut index = if rev { value.last_index() } else { 0 };
    match rev {
        true => for char in value.chars().rev() {
            if char == c { return Some(index) }
            index -= 1;
        },
        false => for char in value.chars() {
            if char == c { return Some(index) }
            index += 1;
        },
    }
    return None
}

impl StrExt for str {
    fn last_index(&self) -> usize {
        self.len() - 1
    }

    fn contains_upper(&self) -> bool {
        // A-Z
        self.chars().any(|it| (65..=90u8).contains(&(it as u8)))
    }

    fn index_of(&self, c: char) -> Option<usize> {
        inner_index_of(self, c, false)
    }

    fn index_of_or(&self, default: usize, c: char) -> usize {
        inner_index_of(self, c, false).unwrap_or(default)
    }

    fn last_index_of(&self, c: char) -> Option<usize> {
        inner_index_of(self, c, true)
    }

    fn last_index_of_or(&self, default: usize, c: char) -> usize {
        inner_index_of(self, c, true).unwrap_or(default)
    }
}

pub trait OptionArg {
    fn some_arg<S: AsRef<OsStr>>(&mut self, arg: Option<S>) -> &mut Self;
}

impl OptionArg for Command {
    fn some_arg<S: AsRef<OsStr>>(&mut self, arg: Option<S>) -> &mut Command {
        match arg {
            None => self,
            Some(arg) => self.arg(arg),
        }
    }
}
