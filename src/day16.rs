use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug)]
enum Instruction{
    Spin(u32),
    Exc(u32,u32),
    Par(char,char)
}

#[aoc_generator(day16)]
fn input_generator(input: &str) -> Vec<Instruction> {
    let mut v:Vec<Instruction>=Vec::new();

//    let input="s1,x3/4,pe/b";

    let res=Regex::new(r"s(\d+)").unwrap();
    let ree=Regex::new(r"x(\d+)\/(\d+)").unwrap();
    let rep=Regex::new(r"p(\w)\/(\w)").unwrap();
    for line in input.split(',') {
        if let Some(caps)=res.captures(line){
            v.push(Instruction::Spin(caps.get(1).unwrap().as_str().parse::<u32>().unwrap()));
        } else if let Some(caps)=ree.captures(line){
            v.push(Instruction::Exc(caps.get(1).unwrap().as_str().parse::<u32>().unwrap(), caps.get(2).unwrap().as_str().parse::<u32>().unwrap()));
        } else if let Some(caps)=rep.captures(line) {
            v.push(Instruction::Par(caps.get(1).unwrap().as_str().chars().next().unwrap(), caps.get(2).unwrap().as_str().chars().next().unwrap()));
        }
    }
//    dbg!(&v);
    v
}

#[aoc(day16, part1)]
fn solve_part1(input: &Vec<Instruction>) -> String {

//    let mut prog="abcde".chars().collect::<Vec<char>>();
    let mut prog="abcdefghijklmnop".chars().collect::<Vec<char>>();

    for ins in input.iter(){
        match ins {
            Instruction::Exc(a, b) => {
                let c=prog[*a as usize];
                prog[*a as usize]=prog[*b as usize];
                prog[*b as usize]=c;
            },  
            Instruction::Par(a, b) => {
                let fa=prog.iter().position(|c| c==a).unwrap();
                let fb=prog.iter().position(|c| c==b).unwrap();
                let c=prog[fa]; prog[fa]=prog[fb];prog[fb]=c;
            },
            Instruction::Spin(a) => {
                for _ in 0..*a as usize {
                    let last=prog.len()-1;
                    let c=prog[last];
                    prog.remove(last);
                    prog.insert(0, c);
                }
            },         
        }
    }
    prog.into_iter().collect()
}

#[aoc(day16, part2)]
fn solve_part2(input: &Vec<Instruction>) -> String {

//    let mut prog="abcde".chars().collect::<Vec<char>>();
    let mut prog="abcdefghijklmnop".chars().collect::<Vec<char>>();


    // as every 30 is resets, 1_000_000_000%30=10
    for t in 0..10 {
        for ins in input.iter(){
            match ins {
                Instruction::Exc(a, b) => {
                    let c=prog[*a as usize];
                    prog[*a as usize]=prog[*b as usize];
                    prog[*b as usize]=c;
                },  
                Instruction::Par(a, b) => {
                    let fa=prog.iter().position(|c| c==a).unwrap();
                    let fb=prog.iter().position(|c| c==b).unwrap();
                    let c=prog[fa]; prog[fa]=prog[fb];prog[fb]=c;
                },
                Instruction::Spin(a) => {
                    for _ in 0..*a as usize {
                        let last=prog.len()-1;
                        let c=prog[last];
                        prog.remove(last);
                        prog.insert(0, c);
                    }
                },         
            }
        }
        println!("It : {t} :{}",prog.clone().into_iter().collect::<String>());
    }
    prog.into_iter().collect()
}