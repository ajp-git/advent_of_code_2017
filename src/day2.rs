use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Vec<u32>> {

    let mut v:Vec<Vec<u32>>=Vec::new();

    for line in input.lines() {
        let s_line=line.split('\t');
        let mut l:Vec<u32>=Vec::new();
        l=s_line.into_iter().map(|v| v.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        l.sort();
        v.push(l);
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
 
#[aoc(day2, part2)]
fn solve_part2(input: &Vec<Vec<u32>>) -> u32 {

    let mut v=input.clone();
    v.iter().map(|l|{
        let mut total=0;
        for i in 0..l.len() {
            for j in i+1..l.len() {
                if l[j]%l[i]==0 {
                    total+=l[j]/l[i];
                    println!("found {} and {} on line {:?}", l[j], l[i], l);
                }
            }
        }
        total
    }).sum()
}
 
