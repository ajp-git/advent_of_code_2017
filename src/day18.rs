use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
enum Data {
    Reg(char),
    Val(i32),
}
#[derive(Debug)]
enum Instruction{
    Snd(char),
    Set(Data,Data),
    Add(Data,Data),
    Mul(Data, Data),
    Mod(Data,Data),
    Rcv(char),
    Jgz(Data,Data),
}

fn parse_ins_data(data: &str) -> Data {
    if let Ok(val) = data.parse::<i32>() {
        Data::Val(val)
    } else if let Some(reg) = data.chars().next() {
        Data::Reg(reg)
    } else {
        panic!("unknown data {}",data);
    }
}

#[aoc_generator(day18)]
fn input_generator(input: &str) -> Vec<Instruction> {
    let mut v:Vec<Instruction>=Vec::new();

    let input:&str="set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";
    for line in input.lines() {
        let parts:Vec<&str>=line.split_whitespace().collect();

        match parts.as_slice() {
            ["snd", x] => v.push(Instruction::Snd(x.chars().next().unwrap())),
            ["add",x,y] => v.push(Instruction::Add(parse_ins_data(&x), parse_ins_data(&y))),
            ["set",x,y] => v.push(Instruction::Set(parse_ins_data(&x), parse_ins_data(&y))),
            ["mul",x,y] => v.push(Instruction::Mul(parse_ins_data(&x), parse_ins_data(&y))),
            ["mod",x,y] => v.push(Instruction::Mod(parse_ins_data(&x), parse_ins_data(&y))),
            ["jgz",x,y] => v.push(Instruction::Jgz(parse_ins_data(&x), parse_ins_data(&y))),
            ["rcv", x] => v.push(Instruction::Rcv(x.chars().next().unwrap())),
            _ => panic!("Unknown instruction {line}"),
        }
    }
    v
}

#[aoc(day18, part1)]
fn solve_part1(input: &Vec<Instruction>) -> u32 {
    let mut pos=0;
    let mut frequency:u32=0;
    let mut last_sound_played:u32=0;
    let mut h_reg:HashMap<char,i32>=HashMap::new();

    loop {
        if pos >= input.len(){
            break;
        }
        let mut jump:usize=0;

        match &input[pos] {
            Instruction::Add(x, y) => {
                match (&x,&y) {
                    (Data::Reg(a),Data::Val(b))=>{
                        let mut c=*b;
                        if let Some(curr)=h_reg.get(&a){
                            c+=curr;
                        }
                        h_reg.insert(*a, c);
                    },
                    __=>panic!("unknown instruction : {:?}", input[pos]),
                }
            },
            Instruction::Mul(x, y) => {
                match (&x,&y) {
                    (Data::Reg(a),Data::Val(b))=>{
                        let mut c=*b;
                        if let Some(curr)=h_reg.get(&a){
                            c*=curr;
                        } 
                        h_reg.insert(*a, c);
                    },
                    (Data::Reg(a),Data::Reg(b))=>{
                        if let Some(c)=h_reg.get(&a) {
                            if let Some(d) = h_reg.get(&b) {
                                let c=c*d;
                                h_reg.insert(*a, c);
                            }
                        }
                    }
                   
                    __=>panic!("unknown instruction : {:?}", input[pos]),
                }
            },
            Instruction::Set(x, y) => {
                match (&x,&y) {
                    (Data::Reg(a),Data::Val(b))=>{
                        h_reg.insert(*a, *b);
                    },
                    __=>panic!("unknown instruction : {:?}", input[pos]),
                }
            },
            Instruction::Rcv(a) => {
                if let Some(c) = h_reg.get(&a) {
                    if *c!=0 {
                        return last_sound_played;
                    }
                }
            },

            Instruction::Snd(a) => {
                if let Some(c)=h_reg.get(&a) {
                    last_sound_played=*c as u32;
                }    
            },
            
            Instruction::Jgz(x, y) => {
                match (&x,&y) {
                    (Data::Reg(a),Data::Val(b))=>{
                        if let Some (c) = h_reg.get(&a) {
                            if *c > 0 {
                                jump=*b as usize;
                            }
                        }
                    },
                    (Data::Reg(a),Data::Reg(b))=>{
                        if let Some (c) = h_reg.get(&a) {
                            if let Some(b) = h_reg.get(&b){
                                if *c > 0 {
                                    jump=*b as usize;
                                }
                            }
                        }
                    },
                    __=>panic!("unknown Instruction::Jgz instruction : {:?}", input[pos]),
                }
            },
        __=>panic!("unknown instruction : {:?}", input[pos]),
        }
        println!("### --- pos+=1+jump;  {:?} \tpos {pos}\t{jump}", &input[pos]);
        println!("{:?}", h_reg);
        println!();
        pos+=1+jump;
    }
0}