use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let marker = fs::read_to_string(file_path)
        .expect("Error reading file")
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .enumerate()
        .find(|(_, cs)| {
            cs[0] != cs[1]
                && cs[1] != cs[2]
                && cs[2] != cs[3]
                && cs[3] != cs[0]
                && cs[0] != cs[2]
                && cs[1] != cs[3]
        })
        .map(|(i, _)| i + 4)
        .expect("Error");

    println!("Marker: {marker}");
}
