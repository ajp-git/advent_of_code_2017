use core::fmt;
use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug, Clone)]
struct Cpu {
    h_slots:HashMap<i32,u8>,
    pos:i32,
    h_states:HashMap<char,State>,
    curr_state:char,
    steps:u32,
    max_steps:u32,
}

impl fmt::Display for Cpu {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\r")?;
        for p in self.pos.saturating_sub(5)..=self.pos.saturating_add(5) {
            let v = self.h_slots.get(&p).unwrap_or(&0);
            if p == self.pos {
                write!(f, "[{v}]")?; // Use the '?' operator to handle errors
            } else {
                write!(f, " {v} ")?; // Use the '?' operator to handle errors
            }
        }

        write!(f, "\tAfter {} steps; About to run state {}", self.steps, self.curr_state)?;
        Ok(()) // Return Ok if everything writes successfully
    }
}

impl Cpu {
    fn run (&mut self) {
        for _ in 0..self.max_steps {
            self.steps+=1;            
            let st = self.h_states.get(&self.curr_state).unwrap();
            
            let mut cur_val:u8=0;
            if self.h_slots.get(&self.pos).is_some(){
                cur_val = *self.h_slots.get(&self.pos).unwrap();
            }
            self.h_slots.insert(self.pos, st.if_write[cur_val as usize]);
            
            self.pos += st.if_jump[cur_val as usize] as i32;
            self.curr_state=st.if_state[cur_val as usize];
            if self.steps%10000==0 {
                print!("{}", self);
            }
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    if_write:[u8;2],
    if_jump:[i8;2],
    if_state:[char;2],
}

#[aoc_generator(day25)]
fn input_generator(input: &str) -> Cpu {


    let mut h_states:HashMap<char, State> = HashMap::new();
    let mut lines=input.lines();
    
    let current_state = lines.next().unwrap().chars().nth(15).unwrap();
    println!("Current state {:?}",current_state);

    let re=Regex::new(r"Perform a diagnostic checksum after (\d+) steps.").unwrap();
    let max_steps=re.captures(lines.next().unwrap()).unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap();
    println!("Max steps {:?}\n",max_steps);


    while lines.next().is_some() { // blank line

        let re=Regex::new(r"In state (\w+).").unwrap();
        let c_state=re.captures(lines.next().unwrap().trim()).unwrap().get(1).unwrap().as_str().chars().nth(0).unwrap();
        println!("Current state {:?}\n",c_state);

        let mut if_write:[u8;2]=[0;2];
        let mut if_jump:[i8;2]=[0;2];
        let mut if_state:[char;2]=[' ';2];

        // - Write the value 1.
        let re_write=Regex::new(r"- Write the value (\d+).").unwrap();
        // - Move one slot to the right.
        let re_jump=Regex::new(r"- Move one slot to the (\w+).").unwrap();
        // - Continue with state B.
        let re_state=Regex::new(r"- Continue with state ([A-Z]).").unwrap();
        
        for i in 0..=1 {
            //If the current value is 0:
            lines.next();

            if_write[i] = re_write.captures(lines.next().unwrap()).unwrap().get(1).unwrap().as_str().parse::<u8>().unwrap();
            println!("if {i} value {:?}\n",if_write[i]);

            let l = lines.next().unwrap();
            match re_jump.captures(l).unwrap().get(1).unwrap().as_str() {
                "left" => if_jump[i]=-1,
                "right" => if_jump[i]=1,
                _ => panic!("{l}"),
            }
            if_state[i]=re_state.captures(lines.next().unwrap()).unwrap().get(1).unwrap().as_str().chars().nth(0).unwrap();
            
            h_states.insert(c_state, State { if_write, if_jump, if_state });

        }
    }

    Cpu{ h_slots: HashMap::new(), curr_state:current_state, h_states, pos:0, steps:0, max_steps }
}

#[aoc(day25, part1)]
fn solve_part1(cpu:&Cpu) -> u32 {

    let mut cpu = cpu.clone();
    println!("Cpu : {:?}", cpu);
    println!();
    println!("Cpu : {}",cpu);
    cpu.run();
    cpu.h_slots.iter().filter(|(_,&v)| v==1).count() as u32
}
