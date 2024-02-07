use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day17)]
fn input_generator(input: &str) -> u32 {
    input.parse::<u32>().unwrap()
}


#[aoc(day17, part1)]
fn solve_part1(input: &u32) -> u32 {
    let input:&u32=&3;
    let mut v:Vec<u32>=vec![0];
    let mut pos=0;
    for i in 1..=2017{
        for _ in 0..(*input as usize){
            pos=(pos+1)%v.len();
        }
        pos+=1;
        if pos==v.len(){
            v.push(i as u32);
        } else {
            v.insert(pos, i as u32);
            
        }
        if i <10 || i>2015{
            println!("i={}\t{pos}\t{:?}", i,v);
        }
    }
    v[pos+1]
}
