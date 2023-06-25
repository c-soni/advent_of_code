use std::env;
use std::fs;

const GRID_SIZE: usize = 99;

fn is_visible(grid: &mut [[u8; GRID_SIZE]; GRID_SIZE], i: usize, j: usize) -> u8 {
    let mut retval = 0;
    let mut vis_top = true;
    let mut vis_bottom = true;
    let mut vis_left = true;
    let mut vis_right = true;

    for r in 0..i {
        if grid[r][j] >= grid[i][j] {
            vis_top = false;
        }
    }
    for r in i + 1..GRID_SIZE {
        if grid[r][j] >= grid[i][j] {
            vis_bottom = false;
        }
    }

    for c in 0..j {
        if grid[i][c] >= grid[i][j] {
            vis_left = false;
        }
    }
    for c in j + 1..GRID_SIZE {
        if grid[i][c] >= grid[i][j] {
            vis_right = false;
        }
    }

    if vis_top || vis_bottom || vis_left || vis_right {
        retval = 1;
    }

    retval
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut grid = [[0 as u8; GRID_SIZE]; GRID_SIZE];
    let mut visible = [[1 as u8; GRID_SIZE]; GRID_SIZE];
    let mut count: u32 = 0;
    fs::read_to_string(file_path)
        .expect("Error reading file")
        .lines()
        .into_iter()
        .enumerate()
        .for_each(|(i, l)| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .into_iter()
                .enumerate()
                .for_each(|(j, b)| grid[i][j] = b as u8)
        });

    for r in 1..GRID_SIZE - 1 {
        for c in 1..GRID_SIZE - 1 {
            visible[r][c] = is_visible(&mut grid, r, c)
        }
    }

    for r in 0..GRID_SIZE {
        for c in 0..GRID_SIZE {
            count += visible[r][c] as u32
        }
    }

    println!("Count {count}");
}
