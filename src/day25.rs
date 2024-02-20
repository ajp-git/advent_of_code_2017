use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};


struct Cpu {
    h_slots:HashMap<i32,u8>,
    pos:i32,
    h_states:HashMap<char,State>,

}

struct State {
    if0_write:u8,
    if0_jump:i8,
    if0_state:char,
    if1_write:u8,
    if1_jump:i8,
    if1_state:char,
}

#[aoc_generator(day25)]
fn input_generator(input: &str) -> HashMap<char, State> {

    let mut h:HashMap<char, State> = HashMap::new();
    let mut lines=input.lines();
    
    let current_state = lines.next().unwrap().chars().nth(15).unwrap();
    println!("Current state {:?}",current_state);

    h
}

#[aoc(day25, part1)]
fn solve_part1(input:&HashMap<char, State>) -> u32 {

0
}
