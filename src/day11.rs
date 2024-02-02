use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day11)]
fn input_generator(input: &str) -> Vec<String> {
    //let input="ne,ne,sw,sw";
    input.split(',').map(|d| d.to_string()).collect::<Vec<String>>()

}
// Thanks to https://www.redblobgames.com/grids/hexagons/#distances !

#[aoc(day11, part1)]
fn solve_part1(input: &Vec<String>) -> u32 {
    
    let mut q:i32=0;
    let mut r:i32=0;
    let mut s:i32=0;

    for d in input{
        match d.as_str() {
            "nw" => {s+=1;r-=1;},        
            "n" => {q+=1;r-=1;},        
            "ne" => {q+=1;s-=1;},        
            "sw" => {q-=1;s+=1;},        
            "s" => {r+=1;q-=1;},        
            "se" => {r+=1;s-=1;}, 
            _ => panic!("unknown direction {d}"),       
        }
        
    }
    (r.abs() as u32+ s.abs() as u32+q.abs() as u32)/2 as u32
}