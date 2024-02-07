use std::{char, collections::HashMap};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy)]
enum Data {
    Reg(char),
    Val(i64),
}
#[derive(Debug, Clone)]

struct Register{
    val:i64,
}
impl Register {
    fn set(&mut self, val: i64){
        self.val=val;
    }
}

#[derive(Debug, Clone)]
enum Instruction{
    Snd(char),
    Set(Data,Data),
    Add(Data,Data),
    Mul(Data, Data),
    Mod(Data,Data),
    Rcv(char),
    Jgz(Data,Data),
}

#[derive(Debug, Clone)]
struct Cpu{
    rom: Vec<Instruction>,
    regs: Vec<Register>,
    counter:i64,
}

impl Cpu {
    fn new (rom:Vec<Instruction>) -> Self {
        let mut regs:Vec<Register>=vec![Register{val:0};26];
        Cpu { rom, regs, counter:0} 
    }
    fn run(&mut self) -> i64 {
        
        loop {
            if self.counter>=self.rom.len() as i64{
                return 0;
            }
            self.execute(self.rom[self.counter as usize].clone());

        }
    }
    fn execute(&mut self, ins: Instruction){
        match ins {
            Instruction::Add(a, b) => {
                println!("a and b Instruction Add {:?}", ins);
                /*self.set(a,
                    self.get(a)
                    + self.get(b) as i64);*/
            },
            _ => panic!("Instruction not handlled {:?} ", ins),
        }
    }
    fn get_pos(&mut self, a:char) -> usize {
        let pos_a: u8 =b'a';
        (a as i64 - pos_a as i64)as usize
    }

    fn set (&mut self, a: Data, b:Data){
        match (a,b) {
            (Data::Reg(a),Data::Reg(_b)) => {
                let pos=self.get_pos(a);
                self.regs[pos].val=self.get(b)
            },
            (Data::Reg(a),Data::Val(b)) => {
                let pos = self.get_pos(a);
                self.regs[pos].val=b;
            },
            _ => panic!("Can't write on val : {:?}", a),      
        }
    }

    fn get (&mut self, a: Data) -> i64 {
        match a {
            Data::Val(a) => a,
            Data::Reg(a) => {
                let pos=self.get_pos(a);
                self.regs[pos].val
            }
        }
    }
}
fn parse_ins_data(data: &str) -> Data {
    if let Ok(val) = data.parse::<i64>() {
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

    /*let input:&str="set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";*/
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
    let mut cpu=Cpu::new(input.clone());
    cpu.run();
    if false {
        
    let mut pos=0;
    let mut frequency:u32=0;
    let mut last_sound_played:u32=0;
    let mut h_reg:HashMap<char,i64>=HashMap::new();

    loop {
        if pos >= input.len(){
            break;
        }
        let mut jump:i64=0;
        println!("Line : {:?}", &input[pos]);
        match &input[pos] {
            Instruction::Add(x, y) => {
                match (&x,&y) {
                    (Data::Reg(a),Data::Val(b))=>{
                        let mut c=0;
                        if let Some(curr)=h_reg.get(&a){
                            c+=curr+*b;
                        }
                        h_reg.insert(*a, c);
                    },
                    __=>panic!("unknown instruction : {:?}", input[pos]),
                }
            },
            
            Instruction::Mod(x, y) => {
                match (&x,&y) {
                    (Data::Reg(a),Data::Val(b))=>{
                        if let Some (v_a) = h_reg.get(&a) {
                            let c = v_a % *b;
                            h_reg.insert(*a, c);                        }
                    },
                    (Data::Reg(a),Data::Reg(b) ) => {
                        if let Some(va)=h_reg.get(&a) {
                            if let Some(b)= h_reg.get(&b) {
                                h_reg.insert(*a,
                                va%b
                                );
                            }
                        }

                    },
                    __=>panic!("unknown Mod instruction : {:?}", input[pos]),
                }
            },
            Instruction::Mul(x, y) => {
                match (&x,&y) {
                    (Data::Reg(a),Data::Val(b))=>{
                        println!("### --- Ins::Mul reg, data : {:?} {:?}",a,b);
                        if let Some(curr)=h_reg.get(&a){
                            let c = curr*b;
                            h_reg.insert(*a, c);
                        } else {
                            println!("{} not found", a);
                            h_reg.insert(*a, 0);

                        }
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
                    println!("---------Playing sound {}\n", last_sound_played);
                }    
            },
            
            Instruction::Jgz(x, y) => {
                match (&x,&y) {
                    (Data::Reg(a),Data::Val(b))=>{
                        if let Some (c) = h_reg.get(&a) {
                            if *c > 0 {
                                jump=*b;
                            }
                        }
                    },
                    (Data::Reg(a),Data::Reg(b))=>{
                        if let Some (c) = h_reg.get(&a) {
                            if let Some(b) = h_reg.get(&b){
                                if *c > 0 {
                                    jump=*b;
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
        if jump != 0 {
            pos=(jump +pos as i64)as usize;
        } else {
            pos+=1;
        }  }

    }0}
