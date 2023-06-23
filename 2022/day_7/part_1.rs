use std::env;
use std::fs;
use std::iter::Skip;

#[macro_use]
extern crate lazy_static;

use regex::Regex;

struct File {
    pub _name: String,
    pub size: u32,
}

struct Directory {
    pub name: String,
    pub dirs: Vec<Directory>,
    pub files: Vec<File>,
    pub total_size: u32,
}

fn traverse(lines_iter: &mut Skip<std::str::Lines>, current_dir: &mut Directory) {
    lazy_static! {
        static ref CDP: Regex = Regex::new(r"cd \.\.").unwrap();
        static ref CD: Regex = Regex::new(r"\$ cd (?<dir_name>[A-Za-z]*)").unwrap();
        static ref FILE: Regex =
            Regex::new(r"(?<file_size>\d+) (?<file_name>[A-Za-z]*[\.A-Za-z]*)").unwrap();
        static ref DIR: Regex = Regex::new(r"dir (?<dir_name>[A-Za-z]*)").unwrap();
    }

    while let Some(line) = lines_iter.next() {
        if CDP.is_match(line) {
            break;
        } else if CD.is_match(line) {
            let caps = CD.captures(line).unwrap();
            let cdir = current_dir
                .dirs
                .iter_mut()
                .find(|dir| dir.name == caps["dir_name"])
                .expect("Error");
            traverse(lines_iter, cdir);
        } else if DIR.is_match(line) {
            let caps = DIR.captures(line).unwrap();
            current_dir.dirs.push(Directory {
                name: caps["dir_name"].to_string(),
                dirs: Vec::new(),
                files: Vec::new(),
                total_size: 0,
            });
        } else if FILE.is_match(line) {
            let caps = FILE.captures(line).unwrap();
            current_dir.files.push(File {
                _name: caps["file_name"].to_string(),
                size: caps["file_size"].parse::<u32>().expect("Error"),
            });
        }
    }
}

fn calculate_sizes(dir: &mut Directory) -> u32 {
    dir.total_size = dir.files.iter().map(|file| file.size).fold(0, |a, b| a + b)
        + dir
            .dirs
            .iter_mut()
            .map(|dir| calculate_sizes(dir))
            .fold(0, |a, b| a + b);
    dir.total_size
}

fn partial_sum(dir: &Directory) -> u32 {
    let mut sum: u32 = 0;
    if dir.total_size <= 100000 {
        sum = dir.total_size;
    }
    sum + dir
        .dirs
        .iter()
        .map(|dir| partial_sum(dir))
        .fold(0, |a, b| a + b)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path).expect("Error reading file");
    let mut lines_iter: Skip<std::str::Lines> = content.lines().skip(1).into_iter();

    let mut root_dir = Directory {
        name: "/".to_string(),
        dirs: Vec::new(),
        files: Vec::new(),
        total_size: 0,
    };
    traverse(&mut lines_iter, &mut root_dir);

    let _x = calculate_sizes(&mut root_dir);
    let sum = partial_sum(&root_dir);
    println!("Partial Total: {sum}");
}
