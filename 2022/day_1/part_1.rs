use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let maximum = fs::read_to_string(file_path)
        .expect("Error reading file")
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|val| {
            val.trim()
                .split("\n")
                .map(|x| x.trim().parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .expect("Error occurred");

    println! {"Max value: {maximum}"}
}
