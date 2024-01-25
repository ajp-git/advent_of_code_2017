use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn input_generator(input: &str) -> u32 {
    input.to_string().parse::<u32>().unwrap()
}

#[aoc(day3, part1)]
fn solve_part1(goal: &u32) -> u32 {
    let mut x:i32=0;
    let mut y:i32=0;
    let mut dx:i32=0;
    let mut dy:i32 = -1;
    let mut step = 0;

    loop {
        step += 1;
        if *goal == step {
            return (x.abs() + y.abs()) as u32;
        }
        if (x == y) || (x < 0 && x == -y) || (x > 0 && x == 1 - y) {
            let temp = dx;
            dx = -dy;
            dy = temp;
        }
        x += dx;
        y += dy;
    }
}

#[aoc(day3, part2)]
fn part2(input: &u32) -> i32 {
    let input:i32=*input as i32;
    let mut grid = Vec::new();
    for _ in 0..1024 {
        grid.push(vec![0; 1024]);
    }
    let mut x = 512;
    let mut y = 512;
    grid[x][y] = 1;
    let mut k = 1;
    loop {
        for _ in 0..k {
            y += 1;
            let r = fill(&mut grid, x, y);
            if r > input {
                return r;
            }
            grid[x][y] = r;
        }
        for _ in 0..k {
            x -= 1;
            let r = fill(&mut grid, x, y);
            if r > input {
                return r;
            }
            grid[x][y] = r;
        }
        k += 1;
        for _ in 0..k {
            y -= 1;
            let r = fill(&mut grid, x, y);
            if r > input {
                return r;
            }
            grid[x][y] = r;
        }
        for _ in 0..k {
            x += 1;
            let r = fill(&mut grid, x, y);
            if r > input {
                return r;
            }
            grid[x][y] = r;
        }
        k += 1;
    }
}

fn fill(grid: &mut Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    grid[x - 1][y - 1] + grid[x][y - 1] + grid[x + 1][y - 1] + grid[x - 1][y] +
    grid[x + 1][y] + grid[x - 1][y + 1] + grid[x][y + 1] + grid[x + 1][y + 1]
}
