use std::fs::create_dir_all;
use std::io;
use std::io::Write;
use std::path::Path;


pub fn print_the_fuck_out() {
    io::stdout().flush().unwrap();
}

pub fn gen_home_path(subpath: Option<&str>) -> String {
    #[allow(deprecated)] // todo replace with a crate
    let mut path = std::env::home_dir().unwrap().to_str().unwrap().to_string();
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

pub fn ensure_dir_exists(path: &str) {
    if !Path::new(path).exists() {
        create_dir_all(path).unwrap();
    }
}

pub fn ensure_parent_exists(path: &String) {
    let parent = Path::new(&path).parent().unwrap();
    create_dir_all(parent).unwrap();
}
