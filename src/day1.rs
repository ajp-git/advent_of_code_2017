use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
fn input_generator(input: &str) -> String {

    for line in input.lines() {
        return line.to_string();
    }
    return "".to_string();
}

#[aoc(day1, part1)]
fn solve_part1(input: &String) -> u32 {

    let mut total=0;
    let mut last_char=input.chars().last().unwrap();
    for c in input.chars() {
        if c==last_char{
            total+=c as u32-'0' as u32;
        }
        last_char=c;
    }
total
}

#[aoc(day1, part2)]
fn solve_part2(input: &String) -> u32 {

    let mut total=0;
    let v:Vec<char>=input.chars().collect_vec();

    for i in 0..v.len() {
        let c=v[i];
        let d=v[(i+v.len()/2)%v.len()];
        if c==d {
            total+=c as u32-'0' as u32;
        }
    }
    total
}
