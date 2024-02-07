/*
Note that you don't need to run both CPUs at the same time 
or even interleaved, unless you are afraid of filling a queue 
with too many values. You can run the first CPU until it blocks, 
then the second CPU until it blocks, then back at the first CPU 
and exit immediately if it still blocks, otherwise continue the cycle.
*/
use std::collections::VecDeque;
use std::str::FromStr;

struct Cpu {
    regs: [i64; 26],
    mem: Vec<Op>,
    pc: usize,
    queue: VecDeque<i64>,
    sent: usize,
    last_sent: i64,
    first_non_zero_recv: i64,
}

impl Cpu {
    fn new(mem: Vec<Op>, id: i64) -> Cpu {
        let mut cpu = Cpu {
            regs: [0; 26],
            mem,
            pc: 0,
            queue: VecDeque::new(),
            sent: 0,
            last_sent: 0,
            first_non_zero_recv: 0,
        };
        cpu.regs[(b'p' - b'a') as usize] = id;
        cpu
    }

    fn run_dual(cpu1: &mut Cpu, cpu2: &mut Cpu) {
        cpu1.run(cpu2, true);
    }

    fn run(&mut self, other: &mut Cpu, first: bool) {
        while self.pc < self.mem.len() {
            let advance = self.mem[self.pc].clone().execute(self, other);
            if !advance {
                if !first {
                    break;
                }
                other.run(self, false);
                if !self.mem[self.pc].clone().execute(self, other) {
                    break;
                }
            }
        }
    }
}

#[derive(Clone)]
enum Value {
    Reg(usize),
    Const(i64),
}

impl Value {
    fn eval(&self, cpu: &Cpu) -> i64 {
        match *self {
            Value::Reg(r) => cpu.regs[r],
            Value::Const(c) => c,
        }
    }

    fn reg(s: &str) -> usize {
        (s.as_bytes()[0] - b'a') as usize
    }
}

impl FromStr for Value {
    type Err = String;

    fn from_str(s: &str) -> Result<Value, String> {
        if s.len() == 1 && s.as_bytes()[0] >= b'a' {
            Ok(Value::Reg(Value::reg(s)))
        } else {
            s.parse()
                .map(Value::Const)
                .map_err(|e| format!("cannot parse {}: {}", s, e))
        }
    }
}

#[derive(Clone)]
enum Op {
    Snd(Value),
    Set(usize, Value),
    Add(usize, Value),
    Mul(usize, Value),
    Mod(usize, Value),
    Rcv(usize),
    Jgz(Value, Value),
}

impl Op {
    fn execute(&self, cpu: &mut Cpu, other: &mut Cpu) -> bool {
        cpu.pc += 1;
        match *self {
            Op::Snd(ref v) => {
                let v = v.eval(cpu);
                cpu.last_sent = v;
                other.queue.push_back(v);
                cpu.sent += 1;
            }
            Op::Set(ref r, ref v) => {
                cpu.regs[*r] = v.eval(cpu);
            }
            Op::Add(ref r, ref v) => {
                cpu.regs[*r] += v.eval(cpu);
            }
            Op::Mul(ref r, ref v) => {
                cpu.regs[*r] *= v.eval(cpu);
            }
            Op::Mod(ref r, ref v) => {
                cpu.regs[*r] %= v.eval(cpu);
                assert!(cpu.regs[*r] >= 0);
            }
            Op::Rcv(ref r) => {
                if cpu.first_non_zero_recv == 0 && cpu.regs[*r] != 0 {
                    cpu.first_non_zero_recv = cpu.last_sent;
                }
                if let Some(v) = cpu.queue.pop_front() {
                    cpu.regs[*r] = v;
                } else {
                    cpu.pc -= 1;
                    return false;
                }
            }
            Op::Jgz(ref t, ref o) => {
                if t.eval(cpu) > 0 {
                    cpu.pc = (cpu.pc as i64 + o.eval(cpu) - 1) as usize;
                }
            }
        }
        true
    }
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Op, String> {
        let words = s.split_whitespace().collect::<Vec<_>>();
        Ok(match words[0] {
            "snd" => Op::Snd(words[1].parse().unwrap()),
            "set" => Op::Set(Value::reg(words[1]), words[2].parse().unwrap()),
            "add" => Op::Add(Value::reg(words[1]), words[2].parse().unwrap()),
            "mul" => Op::Mul(Value::reg(words[1]), words[2].parse().unwrap()),
            "mod" => Op::Mod(Value::reg(words[1]), words[2].parse().unwrap()),
            "rcv" => Op::Rcv(Value::reg(words[1])),
            "jgz" => Op::Jgz(words[1].parse().unwrap(), words[2].parse().unwrap()),
            _ => {
                return Err(format!("cannot parse instruction {}", words[0]));
            }
        })
    }
}

fn main() {
    let mem = include_str!("input")
        .lines()
        .map(|i| i.parse().unwrap())
        .collect::<Vec<Op>>();
    let mut cpu = Cpu::new(mem.clone(), 0);
    let mut other = Cpu::new(mem, 1);
    Cpu::run_dual(&mut cpu, &mut other);
    println!("P1: {}", cpu.first_non_zero_recv);
    println!("P2: {}", other.sent);
}