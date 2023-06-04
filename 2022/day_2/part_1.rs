use std::env;
use std::fs;

#[derive(PartialEq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

struct Game {
    pub opp: Move,
    pub you: Move,
}

fn get_opponent_move(play: &str) -> Move {
    if play == "A" {
        return Move::Rock;
    } else if play == "B" {
        return Move::Paper;
    } else {
        return Move::Scissors;
    }
}

fn get_your_move(play: &str) -> Move {
    if play == "X" {
        return Move::Rock;
    } else if play == "Y" {
        return Move::Paper;
    } else {
        return Move::Scissors;
    }
}

fn calculate_score(game: Game) -> u32 {
    let mut score = 0;
    if game.opp == game.you {
        score = 3;
    } else if game.opp == Move::Rock {
        if game.you == Move::Paper {
            score = 6;
        } else {
            score = 0;
        }
    } else if game.opp == Move::Paper {
        if game.you == Move::Scissors {
            score = 6;
        } else {
            score = 0;
        }
    } else if game.opp == Move::Scissors {
        if game.you == Move::Rock {
            score = 6;
        } else {
            score = 0;
        }
    } else {
        return 0;
    }
    return score + game.you as u32;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let total: u32 = fs::read_to_string(file_path)
        .expect("Error reading file")
        .lines()
        .map(|line| {
            let moves: Vec<&str> = line.split_whitespace().collect();
            calculate_score(Game {
                opp: get_opponent_move(moves[0]),
                you: get_your_move(moves[1]),
            })
        })
        .sum::<u32>();

    println!("Total score: {total}");
}
