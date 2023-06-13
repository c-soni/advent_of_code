use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let marker = fs::read_to_string(file_path)
        .expect("Error reading file")
        .chars()
        .collect::<Vec<char>>()
        .windows(14)
        .enumerate()
        .find(|(_, cs)| cs.iter().collect::<HashSet<&char>>().len() == 14)
        .map(|(i, _)| i + 14)
        .expect("Error");

    println!("Marker: {marker}");
}
