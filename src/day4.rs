use std::collections::{HashMap, HashSet};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{Itertools, Permutations};

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<String> {

    let mut v:Vec<String>=Vec::new();

    for line in input.lines() {
        v.push(line.to_string());
    }
    v
}

#[aoc(day4, part1)]
fn solve_part1(v: &Vec<String>) -> u32 {

    let mut total:u32=0;

    for line in v {
        let mut bgood=true;
        let mut hwords:HashSet<String>=HashSet::new();
        for w in line.split_whitespace() {
            if ! hwords.insert(w.to_string()) {
                bgood=false;
            }
        }
        if bgood {
            total+=1;
        }
    }
 total
}

#[aoc(day4, part2)]
fn solve_part2(v: &Vec<String>) -> u32 {
    let mut total:u32=0;

    let mut total:u32=0;

    for line in v {
        let mut bgood=true;
        let mut hwords:HashSet<String>=HashSet::new();
        for w in line.split_whitespace() {
            let s_word = w.chars().sorted().collect::<String>();

            if ! hwords.insert(s_word) {
                bgood=false;
            }
        }
        if bgood {
            total+=1;
        }
    }
 total
}