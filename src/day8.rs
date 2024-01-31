use std::{cmp::max, collections::{HashMap, HashSet}};
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

/*
uy dec -404 if mmw <= 2
j inc 372 if gk >= -1
uy inc -380 if umr > -4
dy inc 257 if es > -9
 */
#[derive(Debug)]
enum Operation {
    Inc,
    Dec
}

#[derive(Debug)]
enum Test {
    Inf,
    Sup,
    Eq,
    InfEq,
    SupEq,
    Diff,
}
#[derive(Debug)]
struct Instruction {
    register:String,
    ope:Operation,
    val:i32,
    test_reg:String,
    test:Test,
    test_val:i32,
}

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Vec<Instruction> {

    let mut v:Vec<Instruction>=Vec::new();
    let re_disk=Regex::new(r"^(\w+) (inc|dec) (-*\d+) if (\w+) (<|>|<=|>=|==|!=) (-*\d+)$").unwrap();

    for line in input.lines() {
        if let Some(caps)=re_disk.captures(line){
                
            let ins=Instruction{
                register:caps.get(1).unwrap().as_str().to_string(),
                ope: match caps.get(2).unwrap().as_str() {
                    "dec" => Operation::Dec,
                    "inc" => Operation::Inc,
                    _ => panic!("Bad operation on line {}", line),
                },
                val: caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                test_reg: caps.get(4).unwrap().as_str().to_string(),
                test: match caps.get(5).unwrap().as_str() {
                    "<" => Test::Inf,
                    ">" => Test::Sup,
                    "<=" => Test::InfEq,
                    ">=" => Test::SupEq,
                    "!=" => Test::Diff,
                    "==" => Test::Eq,
                    _ => panic!("Bad test in line {}", line),
                },
                test_val:caps.get(6).unwrap().as_str().parse::<i32>().unwrap()
            };
            println!("--Line re: {}", line);
            v.push(ins);

        } else {
            panic!("\tLine not re: {}", line);
        }
    }
    v
}

#[aoc(day8, part1)]
fn solve_part1(input: &Vec<Instruction>) -> i32 {

    let mut h_regs:HashMap<String,i32>=HashMap::new();

    for i in input.iter() {

        let mut reg:i32=0;
        if h_regs.get(&i.register).is_some(){
            reg=*h_regs.get(&i.register).unwrap();
        }

        let mut test_reg:i32=0;
        if h_regs.get(&i.test_reg).is_some(){
            test_reg=*h_regs.get(&i.test_reg).unwrap();
        }
        let mut valid_test=false;

        match i.test {
            Test::Diff => {
                if test_reg != i.test_val {
                    valid_test=true;            
                }
            },
            Test::Eq => {
                if test_reg== i.test_val {
                    valid_test=true;
                }
            },
            Test::Inf => {
                if test_reg < i.test_val {
                    valid_test=true;
                }
            },
            Test::Sup => {
                if test_reg > i.test_val {
                    valid_test=true;
                }
            },
            Test::InfEq => {
                if test_reg <= i.test_val {
                    valid_test=true;
                }
            },
            Test::SupEq => {
                if test_reg >= i.test_val {
                    valid_test=true;
                }
            },
        }

        if valid_test {
            match i.ope {
                Operation::Dec => {
                    reg-=i.val;
                },
                Operation::Inc => {
                    reg+=i.val;
                }
            } 
        }
        h_regs.insert(i.register.clone(), reg);
    }
    dbg!(&h_regs);
    h_regs.iter().map(|(_,&v)|v).max().unwrap()
    
}

#[aoc(day8, part2)]
fn solve_part2(input: &Vec<Instruction>) -> i32 {

    let mut h_regs:HashMap<String,i32>=HashMap::new();
    let mut max_reg:i32=0;

    for i in input.iter() {

        let mut reg:i32=0;
        if h_regs.get(&i.register).is_some(){
            reg=*h_regs.get(&i.register).unwrap();
        }

        let mut test_reg:i32=0;
        if h_regs.get(&i.test_reg).is_some(){
            test_reg=*h_regs.get(&i.test_reg).unwrap();
        }
        let mut valid_test=false;

        match i.test {
            Test::Diff => {
                if test_reg != i.test_val {
                    valid_test=true;            
                }
            },
            Test::Eq => {
                if test_reg== i.test_val {
                    valid_test=true;
                }
            },
            Test::Inf => {
                if test_reg < i.test_val {
                    valid_test=true;
                }
            },
            Test::Sup => {
                if test_reg > i.test_val {
                    valid_test=true;
                }
            },
            Test::InfEq => {
                if test_reg <= i.test_val {
                    valid_test=true;
                }
            },
            Test::SupEq => {
                if test_reg >= i.test_val {
                    valid_test=true;
                }
            },
        }

        if valid_test {
            match i.ope {
                Operation::Dec => {
                    reg-=i.val;
                },
                Operation::Inc => {
                    reg+=i.val;
                }
            } 
        }
        h_regs.insert(i.register.clone(), reg);
        max_reg=max(h_regs.iter().map(|(_,&v)|v).max().unwrap(), max_reg)

    }
    dbg!(&h_regs);
    max_reg
}
