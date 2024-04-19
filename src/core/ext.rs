use std::ffi::OsStr;
use std::fs;
use std::process::{Command, exit, Output};
use crate::core::r#const::ERROR_CODE;


const MORE_THAN_ONE: &str = "adb: more than one device/emulator";
const NO_TARGETS: &str = "adb: no devices/emulators found";

const NBSP: u8 = 0xA0;
const BF: u8 = 0xBF;
const C2: u8 = 0xC2;


pub trait AnyExt<T> {
    fn option(self) -> Option<T>;
}

impl<T> AnyExt<T> for T {
    fn option(self) -> Option<T> { Some(self) }
}

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
    fn index_of_or(&self, default: usize, c: char) -> usize;
    fn last_index_of(&self, c: char) -> Option<usize>;
    fn last_index_of_or(&self, default: usize, c: char) -> usize;
    fn file_name(&self) -> String;
    fn is_file(&self) -> bool;
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

    fn index_of_or(&self, default: usize, c: char) -> usize {
        inner_index_of(self, c, false).unwrap_or(default)
    }

    fn last_index_of(&self, c: char) -> Option<usize> {
        inner_index_of(self, c, true)
    }

    fn last_index_of_or(&self, default: usize, c: char) -> usize {
        inner_index_of(self, c, true).unwrap_or(default)
    }

    fn file_name(&self) -> String {
        let offset = self
            .last_index_of('/')
            .map(|it| it + 1)
            .unwrap_or(0);
        return self.to_string()[offset..].to_string()
    }

    fn is_file(&self) -> bool {
        fs::metadata(self).map_or(false, |it| it.is_file())
    }
}

pub trait StringExt {
    fn with_slash(self) -> Self;
    fn contains_ci(&self, other: &String) -> bool;
}

impl StringExt for String {
    fn with_slash(mut self) -> Self {
        if !self.ends_with('/') {
            self.push('/');
        }
        return self;
    }

    fn contains_ci(&self, other: &String) -> bool {
        self.to_lowercase().contains(other.to_lowercase().as_str())
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
    fn transform<F,R>(&self, f: F) -> Option<R> where F: FnOnce(&T) -> Option<R>;
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
    fn transform<F,R>(&self, f: F) -> Option<R> where F: FnOnce(&T) -> Option<R>{
        match self {
            Some(value) => f(value),
            None => None,
        }
    }
}

