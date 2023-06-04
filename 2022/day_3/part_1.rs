use std::collections::HashMap;
use std::env;
use std::fs;

fn build_priority_map() -> HashMap<char, u32> {
    let mut retval: HashMap<char, u32> = HashMap::new();
    let mut priority = 1;
    let mut char = 'a';
    while char as u8 <= 'z' as u8 {
        retval.insert(char, priority);
        priority += 1;
        char = (char as u8 + 1) as char;
    }
    char = 'A';
    priority = 27;
    while char as u8 <= 'Z' as u8 {
        retval.insert(char, priority);
        priority += 1;
        char = (char as u8 + 1) as char;
    }
    retval
}

fn calculate_priority(line: &str, priority_map: &HashMap<char, u32>) -> u32 {
    let mut priority = 0;
    let length = line.len() / 2;
    let mut table = [0 as u8; 256];
    let left = &line[0..length];
    let right = &line[length..line.len()];
    left.chars().for_each(|c| table[c as usize] = 1);
    right.chars().for_each(|c| {
        if table[c as usize] == 1 {
            priority = *priority_map.get(&c).expect("ERROR");
        }
    });
    priority
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let priority_map = build_priority_map();

    let total: u32 = fs::read_to_string(file_path)
        .expect("Error reading file")
        .lines()
        .map(|line| calculate_priority(line.trim(), &priority_map))
        .sum::<u32>();

    println!("Total score: {total}");
}
