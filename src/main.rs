use std::env;
use std::fs;

#[derive(PartialEq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(PartialEq)]
enum Res {
    Lose,
    Draw,
    Win,
}

struct Game {
    pub opp: Move,
    pub result: Res,
}

fn get_move(play: &str) -> Move {
    if play == "A" {
        return Move::Rock;
    } else if play == "B" {
        return Move::Paper;
    } else {
        return Move::Scissors;
    }
}

fn get_desired_result(res: &str) -> Res {
    if res == "X" {
        return Res::Lose;
    } else if res == "Y" {
        return Res::Draw;
    } else {
        return Res::Win;
    }
}

fn calculate_score(game: Game) -> u32 {
    let round_score = match game.result {
        Res::Lose => 0,
        Res::Draw => 3,
        Res::Win => 6,
    };
    if game.result == Res::Draw {
        return round_score + game.opp as u32;
    } else if game.result == Res::Win {
        if game.opp == Move::Rock {
            return round_score + Move::Paper as u32;
        } else if game.opp == Move::Paper {
            return round_score + Move::Scissors as u32;
        } else {
            return round_score + Move::Rock as u32;
        }
    } else {
        if game.opp == Move::Rock {
            return round_score + Move::Scissors as u32;
        } else if game.opp == Move::Paper {
            return round_score + Move::Rock as u32;
        } else {
            return round_score + Move::Paper as u32;
        }
    }
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
                opp: get_move(moves[0]),
                result: get_desired_result(moves[1]),
            })
        })
        .sum::<u32>();

    println!("Total score: {total}");
}
