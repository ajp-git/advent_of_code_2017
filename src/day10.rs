use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<u32> {
    input.split(',').filter_map(|f|f.trim().parse::<u32>().ok()).collect::<Vec<u32>>()
}
#[derive(Debug,Clone)]
struct Ring<T> {
    data: Vec<T>,
    index: usize,
}

impl<T> Ring<T> {

    fn new () -> Self {
        Ring { data: Vec::new(), index: 0 }
    }

    fn first (&self)-> Option<&T> {
        self.data.get(0)
    }

    fn current(&self) -> Option<&T> {
        self.data.get(self.index)
    }

    fn next(&mut self) {
        self.index=(self.index+1)%self.data.len();
    }
    fn previous(&mut self) {
        self.index=if self.index==0 {self.data.len()-1} else {self.index-1};
    }

    fn get(&self, index:usize) -> Option<&T> {
        self.data.get(index%self.data.len())
    }
    fn put(&mut self, index:usize, t:T) {
        let len=self.data.len();
        self.data[index%len]=t;
    }
    fn push(&mut self, t:T){
        self.data.push(t);
    }
    
    fn rotate_range(&mut self, range: std::ops::Range<usize> ) {
    }

    fn rotate_length_from(&mut self, length:u32, from: u32) {

    }

}
#[aoc(day10, part1)]
fn solve_part1(input: &Vec<u32>) -> u32 {

    let input=vec![3,4,1,5];
    let mut data:Vec<u32>=(0..5).collect();
    
    let mut ring:Ring<u32>=Ring::new();
    for i in 0..5 {
        ring.push(i);
    }

    dbg!(ring);
    0
}
