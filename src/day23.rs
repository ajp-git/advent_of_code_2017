// This module simulates a dual-core CPU execution environment where each CPU can send and receive
// signals to and from each other. It is designed to solve a puzzle from Advent of Code.

// Standard library imports for data structures, threading, timing, and inter-thread communication.
use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};

// Data enum represents a CPU register or a direct value.
#[derive(Debug, Clone)]
enum Data {
    Reg(char),
    Val(i64),
}

// Instruction enum represents the different operations that the CPU can perform.
#[derive(Debug, Clone)]
enum Instruction {
    Set(Data, Data),
    Mul(Data, Data),
    Sub(Data, Data),
    Jnz(Data, Data),
}

/// Represents a single CPU within a simulated dual-core environment.
///
/// The `Cpu` struct is designed to execute a set of instructions (`rom`) and manage its own
/// set of registers (`regs`). It also includes functionality for sending and receiving signals
/// to and from other CPUs via inter-thread communication channels (`sender` and `receiver`).
///
/// Each `Cpu` has a unique identifier (`id`), an instruction pointer (`counter`), and a count
/// of the number of signals it has sent (`sent_counter`).
///
/// # Examples
///
/// ```
/// // Example of creating a new Cpu and running its program.
/// let (sender, receiver) = std::sync::mpsc::channel();
/// let instructions = vec![]; // Vector of Instruction enums representing the program.
/// let mut cpu = Cpu::new(0, instructions, sender, receiver);
/// cpu.run();
/// ```
///
/// # Fields
///
/// - `id`: The unique identifier for the CPU, typically used for differentiating between CPUs in a multi-core setup.
/// - `rom`: A vector of `Instruction` enums representing the program loaded into the CPU's read-only memory.
/// - `regs`: A hashmap representing the CPU's registers, where each key is a character corresponding to the register name.
/// - `counter`: An instruction pointer that tracks the current position within the `rom`.
/// - `receiver`: The receiving end of a channel for inter-CPU communication, used to receive signals from other CPUs.
/// - `sender`: The sending end of a channel for inter-CPU communication, used to send signals to other CPUs.
/// - `sent_counter`: A count of the number of signals this CPU has sent, used for debugging or specific program logic.
#[derive(Debug)]
pub struct Cpu {
    pub id: u32,
    pub rom: Vec<Instruction>,
    pub regs: HashMap<char, i64>,
    pub counter: i64,
    pub mul_counter: i64,
}


impl Cpu {
/// Creates a new `Cpu` instance with the specified ID, program instructions, and communication channels.
///
/// This constructor initializes a `Cpu` with a given identifier, a set of instructions to execute,
/// and channels for sending and receiving signals. It also sets up the initial state of the registers,
/// including setting the 'p' register to the value of the CPU's ID.
///
/// # Parameters
///
/// - `id`: The unique identifier for the CPU. This is also used to initialize the 'p' register.
/// - `rom`: A vector of `Instruction` that the CPU will execute.
/// - `sender`: The sending end of a channel for inter-CPU communication.
/// - `receiver`: The receiving end of a channel for inter-CPU communication.
///
/// # Returns
///
/// Returns a new `Cpu` instance ready to execute the provided instructions and communicate with other CPUs.
///
/// # Examples
///
/// ```
/// let (sender, receiver) = std::sync::mpsc::channel();
/// let instructions = vec![]; // Replace with actual instructions.
/// let cpu = Cpu::new(0, instructions, sender, receiver);
/// ```
pub fn new(id: u32, rom: Vec<Instruction>) -> Self {
    let mut regs: HashMap<char, i64> = HashMap::new();
    regs.insert('p', id as i64);
    let cpu = Cpu {
        id,
        rom,
        regs,
        counter: 0,
        mul_counter: 0,
    };

    cpu
}
    /// Runs the CPU's program until it completes or a condition causes it to exit early.
    pub fn run(&mut self) -> i64 {
        println!("Rom launched on cpu");
        loop {
            if self.counter >= self.rom.len() as i64 {
                return 0;
            }
            let ret = self.execute(&self.rom[self.counter as usize].clone());
            if ret > 0 {
                return ret;
            }
            self.counter += 1;
        }
    }

    // Executes a single instruction and updates the CPU's state accordingly.
    pub fn execute(&mut self, ins: &Instruction) -> i64 {
        // Match on the type of instruction and perform the corresponding operation.

        match ins {
            Instruction::Set(a,b ) => {
                let val_b: i64=self.get(b.clone());
                self.set_val(a.clone(),
                    val_b,
                );

            },
            Instruction::Mul(a,b ) => {
                let val_a: i64=self.get(a.clone());
                let val_b: i64=self.get(b.clone());
                self.set_val(a.clone(),
                    val_a * val_b,
                );
                self.mul_counter+=1
            },
            Instruction::Sub(a,b ) => {
                let val_a: i64=self.get(a.clone());
                let val_b: i64=self.get(b.clone());
                self.set_val(a.clone(),
                    val_a - val_b,
                );

            },
            Instruction::Jnz(a,b ) => {
                let val_a: i64=self.get(a.clone());
                let val_b: i64=self.get(b.clone());
                if val_a!=0{
                    self.counter+=val_b-1;
                }
            },    
        }
        0
    }

    fn set_val (&mut self, a: Data, b:i64){
        if let Data::Reg(reg) = a {
            self.regs.insert(reg, b);
        }
    }
    fn get (&self, a: Data) -> i64 {
        match a {
            Data::Val(a) => a,
            Data::Reg(a) => {
                // Retrieve the value for the register key from the HashMap
                // If the register is not present, return 0 as the default value
                *self.regs.get(&a).unwrap_or(&0)            }
        }
    }
    #[allow(dead_code)]
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
/// Parses a string slice into a `Data` enum.
///
/// This function takes a string slice that represents either an integer value
/// or a register name and attempts to parse it into the corresponding `Data` variant.
/// If the string can be parsed as an `i64` integer, it returns `Data::Val(val)`.
/// If the string cannot be parsed as an integer but contains characters,
/// it assumes the first character is a register name and returns `Data::Reg(reg)`.
///
/// # Arguments
///
/// * `data` - A string slice representing the data to parse.
///
/// # Returns
///
/// A `Data` enum variant corresponding to the parsed value: either `Data::Val` for integers
/// or `Data::Reg` for register names.
///
/// # Panics
///
/// Panics if the input string is empty or cannot be parsed into either an integer or a register name.
///
/// # Examples
///
/// ```
/// let value_data = parse_ins_data("42");
/// assert_eq!(value_data, Data::Val(42));
///
/// let register_data = parse_ins_data("a");
/// assert_eq!(register_data, Data::Reg('a'));
/// ```
///
/// An invalid input, such as an empty string or a string that cannot be parsed, will cause a panic:
///
/// ```should_panic
/// let invalid_data = parse_ins_data(""); // This will panic.
/// ```
pub fn parse_ins_data(data: &str) -> Data {
    if let Ok(val) = data.parse::<i64>() {
        Data::Val(val)
    } else if let Some(reg) = data.chars().next() {
        Data::Reg(reg)
    } else {
        panic!("unknown data {}", data);
    }
}

#[aoc_generator(day23)]
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
    /*let input="snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";*/
    
    for line in input.lines() {
        let parts:Vec<&str>=line.split_whitespace().collect();

        match parts.as_slice() {
            ["sub",x,y] => v.push(Instruction::Sub(parse_ins_data(x), parse_ins_data(y))),
            ["set",x,y] => v.push(Instruction::Set(parse_ins_data(x), parse_ins_data(y))),
            ["mul",x,y] => v.push(Instruction::Mul(parse_ins_data(x), parse_ins_data(y))),
            ["jnz",x,y] => v.push(Instruction::Jnz(parse_ins_data(x), parse_ins_data(y))),
            _ => panic!("Unknown instruction {line}"),
        }
    }
    v
}
#[aoc(day23, part1)]
fn solve_part1(input:&Vec<Instruction>) -> i64 {
    let mut cpu=Cpu::new(0, input.clone());
    cpu.run();

    cpu.mul_counter
}

#[aoc(day23, part2)]
fn solve_part2(input:&Vec<Instruction>) -> i64 {

    let b = 109900;
    let c = 126900;

    let mut res:i64=0;

    for i in (b..(c+1)).step_by(17) {
        if  ! is_prime(i){
    		res += 1;  
        }
    }

	res
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}
