use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day15)]
fn input_generator(input: &str) -> Vec<u128> {
    let mut v:Vec<u128>=Vec::new();

    let re=Regex::new(r"(\d+)").unwrap();
    for line in input.lines() {

        let caps=re.captures(line).unwrap();
        v.push(caps.get(1).unwrap().as_str().parse::<u128>().unwrap());
    }
    dbg!(&v);
    v
}

#[aoc(day15, part1)]
fn solve_part1(input: &Vec<u128>) -> u32 {

    let (mut gen_a, mut gen_b)=(input[0], input[1]);

    let mut count:u32=0;
    
    for i in 0..40_000_000{
        gen_a=(gen_a*16807)%2147483647;
        gen_b=(gen_b*48271)%2147483647;

        let right_a=gen_a&0xffff;
        let right_b=gen_b&0xffff;
        if right_a==right_b{count+=1;}
        if i%100==0{
            print!("\rRound: {i}");
        }
    }
    println!();
    
count
}


#[aoc(day15, part2)]
fn solve_part2(input: &Vec<u128>) -> u32 {

    let (mut gen_a, mut gen_b)=(input[0], input[1]);

    let mut count:u32=0;
    let mut pairs:u128=0;
    
    gen_a=(gen_a*16807)%2147483647;
    gen_b=(gen_b*48271)%2147483647;            
    
    while pairs<5_000_000{
        while gen_a%4!=0{
            gen_a=(gen_a*16807)%2147483647;
        }
        while gen_b%8!=0 {
            gen_b=(gen_b*48271)%2147483647;            
        }
        pairs+=1;

        let right_a=gen_a&0xffff;
        let right_b=gen_b&0xffff;
        if right_a==right_b {
            count+=1;
        }
        if pairs%100==0{
            print!("\rPairs {pairs}");
        }
        gen_a=(gen_a*16807)%2147483647;
        gen_b=(gen_b*48271)%2147483647;            

    }
    println!();
    
count
}