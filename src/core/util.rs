use std::io;
use std::io::Write;
use std::ops::RangeInclusive;
use std::process::exit;

pub const NEW_LINE: char = '\n';
pub const SHELL: &str = "shell";

pub fn exit_err(message: &str) {
    println!("{message}");
    exit(1);
}

pub fn print_the_fuck_out() {
    io::stdout().flush().unwrap();
}

pub fn read_uint(label: &str, default: Option<usize>) -> usize {
    loop {
        let mut input = String::new();

        io::stdout().write(label.as_bytes()).unwrap();

        print_the_fuck_out();

        io::stdin().read_line(&mut input).unwrap();

        if default.is_some() && input.len() == 1 && input.starts_with(NEW_LINE) {
            return default.unwrap();
        }

        match input.trim().parse::<usize>() {
            Ok(value) => return value,
            _ => {}
        }
    }
}

pub fn read_usize_in(label: &str, range: RangeInclusive<usize>) -> usize {
    read_usize(label, None, range)
}

pub fn read_usize_or_in(label: &str, default: usize, range: RangeInclusive<usize>) -> usize {
    read_usize(label, Some(default), range)
}

fn read_usize(label: &str, default: Option<usize>, range: RangeInclusive<usize>) -> usize {
    loop {
        let ans = read_uint(label, default.clone());
        if range.contains(&ans) {
            return ans;
        }
    }
}
