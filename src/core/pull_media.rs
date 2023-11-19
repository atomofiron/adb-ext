use std::cmp::Ordering;
use crate::core::adb_command::AdbArgs;
use crate::core::ext::{OutputExt, VecExt};
use crate::core::selector::{resolve_device, run_adb_with};
use crate::core::r#const::{DASH, DESKTOP_SCREENCASTS, DESKTOP_SCREENSHOTS, NEW_LINE, SHELL, SLASH, SPACE};
use std::process::exit;
use crate::core::strings::{DESTINATION, MEDIAS_NOT_FOUND};
use crate::core::util::{ensure_dir_exists, gen_home_path};

const PULL: &str = "pull";
const LS_SCREENSHOTS: &str = "toybox ls -llcd /sdcard/Pictures/Screenshots/* /sdcard/DCIM/Screenshots/*";
const LS_SCREENCASTS: &str = "toybox ls -llcd /sdcard/Pictures/Screenshots/* /sdcard/DCIM/Screen\\ recordings/* /sdcard/Movies/*";
const PICS: &[&str; 3] = &[".png", ".jpg", ".jpeg"];
const MOVS: &[&str; 3] = &[".mp4", ".mov", ".3gp"];
// -rw-rw---- 1 u0_a173 media_rw 6217184 2023-10-23 00:15:07.020796477 +0200(wtf?) /sdcard/Pictures/Screenshots/screenshot.png
const PART_MIN_COUNT: usize = 8;
const PART_DATE: usize = 5;
const PART_TIME: usize = 6;

pub fn pull_screenshots(count: usize) {
    pull(count, PICS, &[SHELL, LS_SCREENSHOTS], DESKTOP_SCREENSHOTS);
}

pub fn pull_screencasts(count: usize) {
    pull(count, MOVS, &[SHELL, LS_SCREENCASTS], DESKTOP_SCREENCASTS);
}

fn pull(count: usize, exts: &[&str], args: &[&str], target: &str) {
    if count <= 0 {
        return
    }
    let device = resolve_device();
    let output = run_adb_with(&device, AdbArgs::run(args));
    let mut items = output.stdout().split(NEW_LINE)
        .into_iter()
        .map(|it| splitn_by(it, PART_MIN_COUNT, SPACE))
        .filter_map(|it| as_item_or_none(exts, it))
        .collect::<Vec<Item>>();
    if items.is_empty() {
        let err = output.stderr();
        if err.is_empty() {
            MEDIAS_NOT_FOUND.println();
        } else {
            println!("{}", output.stderr());
            exit(output.code());
        }
    } else {
        items.sort();
        let mut items = items.iter()
            .take(count)
            .map(|it| it.path.to_string())
            .collect::<Vec<String>>();
        let mut pull_args = AdbArgs::run(&[PULL]);
        pull_args.args.append(&mut items);
        let dst = gen_home_path(Some(target));
        ensure_dir_exists(&dst);
        pull_args.args.push(dst.clone());
        let output = run_adb_with(&device, pull_args);
        output.print_out_and_err();
        if output.status.success() {
            DESTINATION.print();
            println!("{dst}");
        }
        exit(output.code());
    }
}

fn as_item_or_none(exts: &[&str], line: Vec<String>) -> Option<Item> {
    match () {
        _ if line.len() < PART_MIN_COUNT => return None,
        _ if line[0].chars().next() != Some(DASH) => return None,
        _ => (),
    }
    let last = line[line.last_index()].to_string();
    let count = 5.min(last.len());
    let ext = last[(last.len() - count)..].to_lowercase();
    if !exts.iter().any(|it| ext.ends_with(it)) {
        return None
    }
    let date = line[PART_DATE].to_string();
    let time = line[PART_TIME].to_string();
    let root = last.chars().position(|c| c == SLASH).unwrap();
    // ignore the part contains '+0200' if exists
    let path = last[root..].to_string();
    Some(Item { date, time, path })
}

fn splitn_by(str: &str, limit: usize, sep: char) -> Vec<String> {
    let mut parts: Vec<String> = vec![];
    let mut buf: Vec<char> = vec![];
    let release_part = |buf: &mut Vec<char>, parts: &mut Vec<String>| {
        parts.push(buf.clone().into_iter().collect::<String>());
        buf.clear();
    };
    for c in str.chars() {
        if c != sep || parts.len() == (limit - 1) {
            buf.push(c);
        } else if !buf.is_empty() {
            release_part(&mut buf, &mut parts);
        }
    }
    if !buf.is_empty() {
        release_part(&mut buf, &mut parts);
    }
    return parts;
}

struct Item {
    pub date: String,
    pub time: String,
    pub path: String,
}

impl Eq for Item {}

impl PartialEq<Self> for Item {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date && self.time == other.time
    }
}

impl PartialOrd<Self> for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match () {
            _ if self.date > other.date => Ordering::Less,
            _ if self.date < other.date => Ordering::Greater,
            _ if self.time > other.time => Ordering::Less,
            _ if self.time < other.time => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}
