use std::env;
use std::fs;

fn check_overlap(first: (u32, u32), second: (u32, u32)) -> u32 {
    if first.0 >= second.0 && first.0 <= second.1
        || first.1 >= second.0 && first.1 <= second.1
        || second.0 >= first.0 && second.0 <= first.1
        || second.1 >= first.0 && second.1 <= first.1
    {
        return 1;
    } else {
        return 0;
    }
}

fn count_pairs(line: &str) -> u32 {
    let pairs: Vec<(u32, u32)> = line
        .split(",")
        .map(|val| {
            let pair: Vec<u32> = val
                .split("-")
                .map(|x| x.parse::<u32>().expect("ERROR"))
                .collect();
            (pair[0], pair[1])
        })
        .collect();
    check_overlap(pairs[0], pairs[1])
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let total: u32 = fs::read_to_string(file_path)
        .expect("Error reading file")
        .lines()
        .map(|line| count_pairs(line.trim()))
        .sum::<u32>();

    println!("Total score: {total}");
}
