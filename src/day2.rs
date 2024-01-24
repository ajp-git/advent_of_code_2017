use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Vec<u32>> {

    let mut v:Vec<Vec<u32>>=Vec::new();

    for line in input.lines() {
        let s_line=line.split('\t');
        v.push(s_line.into_iter().map(|v| v.parse::<u32>().unwrap()).collect::<Vec<u32>>());
    }
    println!("{:?}", v);
    v
}

#[aoc(day2, part1)]
fn solve_part1(input: &Vec<Vec<u32>>) -> u32 {

    input.iter().map(|l|{
        l.iter().max().unwrap()-l.iter().min().unwrap()
    }).sum()
}

