use std::io;
use std::io::Write;
use std::ops::RangeInclusive;
use std::process::exit;

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

pub fn print_the_fuck_out() {
    io::stdout()
        .flush()
        .unwrap();
}

pub fn read_uint(label: &str) -> u32 {
    let mut input = String::new();
    loop {
        input.clear();

        io::stdout()
            .write(label.as_bytes())
            .unwrap();

        print_the_fuck_out();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<u32>() {
            Ok(value) => return value,
            _ => {},
        }
    }
}

pub fn read_usize_in(label: &str, range: RangeInclusive<usize>) -> usize {
    loop {
        let ans = read_uint(label) as usize;
        if range.contains(&ans) {
            return ans;
        }
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
