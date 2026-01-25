use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::{Command, Output};
use crate::core::r#const::ERROR_CODE;

const NO_TARGETS: &str = "adb: no devices/emulators found";

const NBSP: u8 = 0xA0;
const BF: u8 = 0xBF;
const C2: u8 = 0xC2;

pub trait OutputExt {
    fn code(&self) -> i32;
    fn stdout(&self) -> String;
    fn stderr(&self) -> String;
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

pub trait VecExt {
    type Item: PartialEq;
    fn last_index(&self) -> usize;
    fn index_of(&self, target: &Self::Item) -> Option<usize>;
}

impl<T> VecExt for Vec<T> where T: PartialEq {
    type Item = T;

    fn last_index(&self) -> usize {
        self.len() - 1
    }

    fn index_of(&self, target: &T) -> Option<usize> {
        for (i, it) in self.iter().enumerate() {
            if it.eq(target) {
                return Some(i);
            }
        }
        return None;
    }
}

pub trait StrExt {
    fn last_index(&self) -> usize;
    fn index_of(&self, c: char) -> Option<usize>;
    fn last_index_of(&self, c: char) -> Option<usize>;
    fn file_name(&self) -> String;
    fn path(&self) -> PathBuf;
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

    fn index_of(&self, c: char) -> Option<usize> {
        inner_index_of(self, c, false)
    }

    fn last_index_of(&self, c: char) -> Option<usize> {
        inner_index_of(self, c, true)
    }

    fn file_name(&self) -> String {
        let offset = self
            .last_index_of('/')
            .map(|it| it + 1)
            .unwrap_or(0);
        return self.to_string()[offset..].to_string()
    }

    fn path(&self) -> PathBuf {
        PathBuf::from(self)
    }
}

pub trait StringExt {
    fn contains_ci(&self, other: &String) -> bool;
    fn path(&self) -> PathBuf;
}

impl StringExt for String {

    fn contains_ci(&self, other: &String) -> bool {
        self.to_lowercase().contains(other.to_lowercase().as_str())
    }

    fn path(&self) -> PathBuf {
        PathBuf::from(self)
    }
}

pub trait PathBufExt {
    fn to_string(&self) -> String;
}

impl PathBufExt for PathBuf {
    fn to_string(&self) -> String {
        self.to_string_lossy().to_string()
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

pub trait OptionExt<T> {
    fn take_some_if<F>(self, f: F) -> Option<T> where F: FnOnce(&T) -> bool;
    fn if_none<F>(self, f: F) -> Option<T> where F: FnOnce() -> Option<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn take_some_if<F>(self, f: F) -> Option<T> where F: FnOnce(&T) -> bool {
        match &self {
            None => self,
            Some(value) if f(value) => self,
            _ => None,
        }
    }
    fn if_none<F>(self, f: F) -> Option<T> where F: FnOnce() -> Option<T> {
        match self {
            None => f(),
            Some(_) => self,
        }
    }
}

