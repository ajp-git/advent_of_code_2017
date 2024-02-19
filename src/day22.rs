use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};


#[aoc_generator(day22)]
fn input_generator(input: &str) -> HashMap<(i32,i32), bool> {

    let mut h:HashMap<(i32,i32), bool> = HashMap::new();
    let mut pos_x = 0;
    let mut pos_y = 0;

    for line in input.lines() {
        pos_x=0;
        for c in line.chars() {
            h.insert((pos_x,pos_y), if c == '#' { true } else { false});
            pos_x+=1;
        }
        pos_y+=1;
    }

    println!("{:?}", h);
    h
}


#[aoc(day22, part1)]
fn solve_part1(h: &HashMap<(i32,i32), bool>) -> u32 {

    // find middle to start
    let start_x = (h.keys().map(|(x,_)| x ).max().unwrap()-
        h.keys().map(|(x,_)| x ).min().unwrap())/2+1;

    let start_y = (h.keys().map(|(_,y)| y ).max().unwrap()-
        h.keys().map(|(_,y)| y ).min().unwrap())/2+1;

    println!("Starting at ({},{})", start_x, start_y);
    0
}