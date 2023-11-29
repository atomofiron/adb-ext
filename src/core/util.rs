use std::fs::create_dir_all;
use std::io;
use std::io::Write;
use std::path::Path;


pub fn print_the_fuck_out() {
    io::stdout().flush().unwrap();
}

pub fn home_dir() -> String {
    #[allow(deprecated)] // todo replace with a crate
    std::env::home_dir().unwrap().to_str().unwrap().to_string()
}

pub fn gen_home_path(subpath: Option<&str>) -> String {
    let mut path = home_dir();
    if let Some(subpath) = subpath {
        if !subpath.starts_with('/') {
            path.push('/');
        }
        path.push_str(subpath);
    } else {
        path.push('/');
    }
    return path;
}

pub fn ensure_parent_exists(path: &String) {
    let parent = Path::new(&path).parent().unwrap();
    create_dir_all(parent).unwrap();
}
