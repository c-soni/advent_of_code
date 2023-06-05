use std::collections::HashMap;
use std::collections::HashSet;
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

fn get_badge_priority(
    i: usize,
    line: &str,
    table: &mut [u8; 256],
    priority_map: &HashMap<char, u32>,
) -> u32 {
    let mut badge_priority = 0;
    line.chars()
        .collect::<HashSet<_>>()
        .iter()
        .for_each(|&c| table[c as usize] += 1);
    if i % 3 == 2 {
        let x = ((table.iter().position(|&x| x == 3).expect("Not found") as u32) as u8) as char;
        badge_priority = *priority_map.get(&x).expect("Error fetching value");
        table.fill(0)
    }
    badge_priority
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let priority_map = build_priority_map();
    let mut table = [0 as u8; 256];

    let total: u32 = fs::read_to_string(file_path)
        .expect("Error reading file")
        .lines()
        .enumerate()
        .map(|(i, line)| get_badge_priority(i, line.trim(), &mut table, &priority_map))
        .sum::<u32>();

    println!("Total score: {total}");
}
