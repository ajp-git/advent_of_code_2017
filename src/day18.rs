use std::{char};
use std::sync::mpsc::{self, Sender, Receiver};


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

#[derive(Debug, Clone)]
enum Instruction{
    Snd(Data),
    Set(Data,Data),
    Add(Data,Data),
    Mul(Data, Data),
    Mod(Data,Data),
    Rcv(Data),
    Jgz(Data,Data),
}

#[derive(Debug, Clone)]
struct Cpu{
    rom: Vec<Instruction>,
    regs: Vec<Register>,
    counter:i64,
    last_sound:i64,
}

impl Cpu {
    fn new (rom:Vec<Instruction>) -> Self {
        let regs:Vec<Register>=vec![Register{val:0};26];
        Cpu { rom, regs, counter:0, last_sound:0} 
    }
    fn run(&mut self) -> i64 {
        
        loop {
            if self.counter>=self.rom.len() as i64{
                return 0;
            }
            let ret = self.execute(self.rom[self.counter as usize].clone());
            if ret>0{
                return ret;
            }
            self.counter+=1;

        }
    }
    fn execute(&mut self, ins: Instruction) -> i64{
        self.display_registers();
        println!("{ins:?}");
        match ins {
            Instruction::Add(a, b) => {
                let val_a: i64=self.get(a);
                let val_b: i64=self.get(b);
                self.set_val(a,
                    val_b+val_a,
                );
            },
            Instruction::Set(a,b ) => {
                let val_b: i64=self.get(b);
                self.set_val(a,
                    val_b,
                );

            },
            Instruction::Mul(a,b ) => {
                let val_a: i64=self.get(a);
                let val_b: i64=self.get(b);
                self.set_val(a,
                    val_a * val_b,
                );

            },
            Instruction::Mod(a,b ) => {
                let val_a: i64=self.get(a);
                let val_b: i64=self.get(b);
                self.set_val(a,
                    val_a % val_b,
                );
            },
            Instruction::Jgz(a,b ) => {
                let val_a: i64=self.get(a);
                let val_b: i64=self.get(b);
                if val_a>0{
                    self.counter+=val_b-1;
                }
            },    
            Instruction::Snd(a ) => {
                let val_a: i64=self.get(a);
                println!("Playing sound {}",val_a);
                self.last_sound=val_a;
            },
            Instruction::Rcv(a ) => {
                let val_a: i64=self.get(a);
                println!("{} should be > 0",val_a);
                if val_a>0{
                    println!("And last sound {}",self.last_sound);
                    return self.last_sound;    
                }
            },
        }
        0
    }
    fn get_pos(&mut self, a:char) -> usize {
        let pos_a: u8 =b'a';
        (a as i64 - pos_a as i64)as usize
    }

    fn set_val (&mut self, a: Data, b:i64){
        match (a,b) {
            (Data::Reg(a),b) => {
                let pos = self.get_pos(a);
                self.regs[pos].val=b;
            },
            _ => panic!("Can't write on val : {:?}", a),      
        }
    }

    /*fn set_data (&mut self, a: Data, b:Data){
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
    }*/

    fn get (&mut self, a: Data) -> i64 {
        match a {
            Data::Val(a) => a,
            Data::Reg(a) => {
                let pos=self.get_pos(a);
                self.regs[pos].val
            }
        }
    }
    fn display_registers(&mut self){
        for r in 'a'..='p' {
            if r=='h'{
                println!();
            }
            print!("{r} : {} -- ",self.get(Data::Reg(r)));
        }
        println!();

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
            ["snd", x] => v.push(Instruction::Snd(parse_ins_data(x))),
            ["add",x,y] => v.push(Instruction::Add(parse_ins_data(x), parse_ins_data(y))),
            ["set",x,y] => v.push(Instruction::Set(parse_ins_data(x), parse_ins_data(y))),
            ["mul",x,y] => v.push(Instruction::Mul(parse_ins_data(x), parse_ins_data(y))),
            ["mod",x,y] => v.push(Instruction::Mod(parse_ins_data(x), parse_ins_data(y))),
            ["jgz",x,y] => v.push(Instruction::Jgz(parse_ins_data(x), parse_ins_data(y))),
            ["rcv", x] => v.push(Instruction::Rcv(parse_ins_data(x))),
            _ => panic!("Unknown instruction {line}"),
        }
    }
    v
}

#[aoc(day18, part1)]
fn solve_part1(input:&[Instruction]) -> i64 {
    let mut cpu=Cpu::new(input.to_owned());
    cpu.run()
}
