use std::collections::{HashMap, HashSet};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{Itertools, Permutations};

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<i32> {

    let mut v:Vec<i32>=Vec::new();

    for line in input.lines() {
        v.push(line.to_string().parse::<i32>().unwrap());
    }
    v
}

#[aoc(day5, part1)]
fn solve_part1(v: &Vec<i32>) -> u32 {
    let mut j = v.clone();
    let mut addr=0;
    let mut steps:u32=0;

    loop {
        steps+=1;

        let old_addr = addr;
        addr+=j[old_addr as usize];
        j[old_addr as usize]+=1;

        if addr<0 || addr>=j.len() as i32{
            break;
        }
    }
    steps
}
#[aoc(day5, part2)]
fn solve_part2(v: &Vec<i32>) -> u32 {
    let mut j = v.clone();
    let mut addr=0;
    let mut steps:u32=0;

    loop {
        steps+=1;

        let old_addr = addr;
        let jump=j[old_addr as usize];
        addr+=j[old_addr as usize];
        
        if jump >=3 {
            j[old_addr as usize]-=1;

        } else {
            j[old_addr as usize]+=1;
        }

        if addr<0 || addr>=j.len() as i32{
            break;
        }
    }
    steps
}
