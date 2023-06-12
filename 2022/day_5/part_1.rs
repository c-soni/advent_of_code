use std::collections::VecDeque;
use std::env;
use std::fs;

use regex::Regex;

struct Move {
    pub count: u32,
    pub from: usize,
    pub to: usize,
}

fn setup_starting_state(state: &mut Vec<VecDeque<char>>, starting_point: &[&str]) {
    starting_point.iter().for_each(|&line| {
        let v: Vec<_> = line.chars().collect();
        v.chunks(4).enumerate().for_each(|(c, ch)| {
            if ch[1].is_ascii_uppercase() {
                state[c].push_back(ch[1]);
            }
        });
    });
}

fn get_moves(moves: &mut Vec<Move>, moves_str: &[&str]) {
    let move_re = Regex::new(r"move (?<c>\d+) from (?<f>\d+) to (?<t>\d+)").unwrap();
    for &some_move in moves_str {
        let caps = move_re.captures(some_move).unwrap();
        moves.push(Move {
            count: caps["c"].parse::<u32>().expect("ERROR"),
            from: caps["f"].parse::<usize>().expect("ERROR") - 1,
            to: caps["t"].parse::<usize>().expect("ERROR") - 1,
        });
    }
}

fn get_string(state: &mut Vec<VecDeque<char>>, moves: &Vec<Move>) -> String {
    moves.iter().for_each(|m| {
        for _ in 0..m.count {
            let c = state[m.from].pop_front().expect("ERROR");
            state[m.to].push_front(c);
        }
    });
    state.into_iter().map(|v| v[0]).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path).expect("Error reading file");
    let lines: Vec<_> = content.lines().collect();
    let partition: usize = lines.iter().position(|&val| val == "").expect("ERROR");

    let starting_point = &lines[..partition - 1];
    let moves_str = &lines[partition + 1..];
    let num_piles = lines[partition - 1]
        .split_whitespace()
        .map(|x| x.trim().parse::<u32>().expect("ERROR"))
        .last()
        .expect("ERROR");

    let mut state: Vec<VecDeque<char>> = Vec::new();
    for _ in 0..num_piles {
        state.push(VecDeque::new());
    }

    setup_starting_state(&mut state, starting_point);

    let mut moves: Vec<Move> = Vec::new();
    get_moves(&mut moves, moves_str);

    println!("{}", get_string(&mut state, &moves));
}
