pub mod command;
pub mod exit_status;
pub mod option;
pub mod output;
pub mod path_buf;
pub mod print;
pub mod result;
pub mod str;
pub mod string;
pub mod trim;
pub mod vec;

use crate::core::ext::result::ResultExt;
use std::error::Error;
use std::fmt::Display;
use std::io::Write;
use std::ops::Range;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

const NBSP: u8 = 0xA0;
const BF: u8 = 0xBF;
const C2: u8 = 0xC2;

pub type Rslt<T> = Result<T, Box<dyn Error>>;

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

pub fn take_if<T, F: Fn(&T) -> bool>(value: T, predicate: F) -> Option<T> {
    match predicate(&value) {
        true => Some(value),
        false => None,
    }
}

