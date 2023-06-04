use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut all_totals: Vec<u32> = fs::read_to_string(file_path)
        .expect("Error reading file")
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|val| {
            val.trim()
                .split("\n")
                .map(|x| x.trim().parse::<u32>().unwrap())
                .sum::<u32>()
        }).collect();

    all_totals.sort_unstable_by(|a, b| b.cmp(a));

    let sum_top_three = all_totals[0..3].iter().sum::<u32>();
    println!("Total of top 3: {sum_top_three}");
}
