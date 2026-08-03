use std::fmt::Display;
use std::io;
use std::io::Write;
use crate::core::ext::eprintln;

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
