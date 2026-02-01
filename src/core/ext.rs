use crate::core::r#const::{ERROR_CODE, NULL};
use std::error::Error;
use std::ffi::OsStr;
use std::fmt::Display;
use std::io;
use std::io::Write;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Output};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

const NO_TARGETS: &str = "adb: no devices/emulators found";

const NBSP: u8 = 0xA0;
const BF: u8 = 0xBF;
const C2: u8 = 0xC2;

pub type Rslt<T> = Result<T, Box<dyn Error>>;

pub trait OutputExt {
    fn exit_code(&self) -> ExitCode;
    fn stdout(&self) -> String;
    fn stderr(&self) -> String;
    fn print_out(&self);
    fn print_err(&self);
    fn print_out_and_err(&self);
}

impl OutputExt for Output {
    fn exit_code(&self) -> ExitCode {
        let code = self.status.code().unwrap_or(ERROR_CODE) & 255;
        ExitCode::from(code as u8)
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
            stdout.println()
        }
    }
    fn print_err(&self) {
        let stderr = self.stderr();
        if !stderr.is_empty() {
            stderr.eprintln();
        }
    }
    fn print_out_and_err(&self) {
        self.print_out();
        self.print_err();
    }
}

pub trait ResultExt<R, E> {
    fn string_err(self) -> Result<R, String>;
    fn soft_unwrap(self) -> Option<R>;
    fn boxed(self) -> Result<R, Box<dyn Error>> where E: Error + Send + Sync + 'static;
}

impl<R, E> ResultExt<R, E> for Result<R, E> where E: Display {

    fn string_err(self) -> Result<R, String> {
        self.map_err(|e| e.to_string())
    }

    fn soft_unwrap(self) -> Option<R> {
        if let Err(e) = &self {
            e.eprintln();
        }
        self.ok()
    }

    fn boxed(self) -> Result<R, Box<dyn Error>> where E: Error + Send + Sync + 'static {
        self.map_err(Into::into)
    }
}

pub fn print_no_one() {
    NO_TARGETS.println();
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

pub trait PrintExt {
    fn print(&self);
    fn println(&self);
    fn eprintln(&self);
}

impl<E: Display> PrintExt for E {

    fn print(&self) {
        print!("{self}");
        io::stdout().flush().unwrap();
    }

    fn println(&self) {
        println!("{self}");
    }

    fn eprintln(&self) {
        if let Err(_) = eprintln(self) {
            #[cfg(unix)]
            eprintln!("\x1b[31m{self}\x1b[0m");
            #[cfg(windows)]
            self.eprintln();
        }
    }
}

fn eprintln<E: Display>(e: &E) -> Rslt<()> {
    let mut stream = StandardStream::stderr(ColorChoice::Auto);
    stream.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
    writeln!(&mut stream, "{e}")?;
    return stream.reset().boxed()
}

pub fn try_make_colored<'l,>(value: &str, color: Color, range: Range<usize>) -> String {
    match make_colored(&value, color, range) {
        Ok(colored) => colored,
        Err(_) => value.to_string(),
    }
}

fn make_colored<'l>(value: &str, color: Color, range: Range<usize>) -> Rslt<String> {
    let bw = BufferWriter::stderr(ColorChoice::AlwaysAnsi);
    let mut buf = bw.buffer();
    buf.set_color(ColorSpec::new().set_fg(Some(color)))?;

    let mut result = String::new();
    result.push_str(&value[..range.start]);

    write!(&mut buf, "{}", &value[range.clone()])?;
    buf.reset()?;
    let colored = String::from_utf8_lossy(buf.as_slice()).into_owned();
    result.push_str(&colored);

    result.push_str(&value[range.end..]);
    return Ok(result)
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

pub trait VecExt<T> {
    fn last_index(&self) -> usize;
    fn index_of<P: Fn(&T) -> bool>(&self, predicate: P) -> Option<usize>;
    fn try_remove(&mut self, index: usize) -> Option<T>;
}

impl<T> VecExt<T> for Vec<T> {

    fn last_index(&self) -> usize {
        self.len() - 1
    }

    fn index_of<P: Fn(&T) -> bool>(&self, predicate: P) -> Option<usize> {
        for (i, it) in self.iter().enumerate() {
            if predicate(it) {
                return Some(i);
            }
        }
        return None;
    }

    fn try_remove(&mut self, index: usize) -> Option<T> {
        match () {
            _ if index < self.len() => Some(self.remove(index)),
            _ => None,
        }
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
    fn is_null_or_empty(&self) -> bool;
}

impl StringExt for String {

    fn contains_ci(&self, other: &String) -> bool {
        self.to_lowercase().contains(other.to_lowercase().as_str())
    }

    fn path(&self) -> PathBuf {
        PathBuf::from(self)
    }

    fn is_null_or_empty(&self) -> bool {
        self.is_empty() || self == NULL
    }
}

pub trait PathBufExt {
    fn to_string(&self) -> String;
    fn to_str(&self) -> &str;
    fn is_null_or_empty(&self) -> bool;
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
    fn if_some<F>(self, f: F) -> Option<T> where F: FnOnce(&T);
    fn if_none<F>(self, f: F) -> Option<T> where F: FnOnce();
}

impl<T> OptionExt<T> for Option<T> {
    fn take_some_if<F>(self, f: F) -> Option<T> where F: FnOnce(&T) -> bool {
        match &self {
            None => self,
            Some(value) if f(value) => self,
            _ => None,
        }
    }
    fn if_some<F>(self, f: F) -> Option<T> where F: FnOnce(&T) {
        match &self {
            Some(value) => f(value),
            None => (),
        }
        return self
    }
    fn if_none<F>(self, f: F) -> Option<T> where F: FnOnce() {
        match &self {
            None => f(),
            Some(_) => (),
        }
        return self
    }
}

pub fn take_if<T, F: Fn(&T) -> bool>(value: T, predicate: F) -> Option<T> {
    match predicate(&value) {
        true => Some(value),
        false => None,
    }
}

