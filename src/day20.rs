use std::{cmp::{max, min}, thread::sleep, time::Duration};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Coord {
    x:i32,
    y:i32,
    z:i32,
}

impl Coord {
    fn new (s:String) -> Self{
        let mut s_s=s.split(',');
        //println!("Coord : {}", s);
        let mut c = Coord {
            x:s_s.next().unwrap().parse::<i32>().unwrap(),
            y:s_s.next().unwrap().parse::<i32>().unwrap(),
            z:s_s.next().unwrap().chars().filter(|c|c.is_digit(10)).collect::<String>().parse::<i32>().unwrap(),
        };
        c       
    }
}

#[derive(Debug, Clone)]
struct Particle {
    pos:Coord,
    vel:Coord,
    acc:Coord
}

impl Particle {
    fn new (s:String) -> Self {
        let mut s_s=s.split('<');
       // println!("Vector : {}", s);
        s_s.next();
        let p = Particle {
            pos:Coord::new(s_s.next().unwrap().to_string()),
            vel:Coord::new(s_s.next().unwrap().to_string()),
            acc:Coord::new(s_s.next().unwrap().to_string()),
        };
        p
    }
}
impl Particle {
    fn distance(&self)->usize {
        (self.pos.x.abs()+self.pos.y.abs()+self.pos.z.abs()) as usize
    }
    fn run (&mut self) {
        self.vel.x+=self.acc.x;
        self.vel.y+=self.acc.y;
        self.vel.z+=self.acc.z;

        self.pos.x+=self.vel.x;
        self.pos.y+=self.vel.y;
        self.pos.z+=self.vel.z;        
    }
}

#[aoc_generator(day20)]
fn input_generator(input: &str) -> Vec<Particle> {
    let mut v:Vec<Particle>=Vec::new();
    for line in input.lines() {
        let line = line.to_string();
        v.push(Particle::new(line));
    }
//    dbg!(&v);
    v
}


#[aoc(day20, part1)]
fn solve_part1(input: &Vec<Particle>) -> String {
    let mut v = input.clone(); // Clone input into a new Vec<Particle>

    for i in 0..100000 {
        v.iter_mut().for_each(|p| p.run());

        let m= v.iter()
            .enumerate()
            .map(
                |(u,p)|{
                    (u,p.distance()) })
            .min_by_key(|&(_u, distance)| distance)
            .unwrap(); // Compare by distance

        print!("\r{i}\tm : {:?}", m);
    }
    println!();
"".to_string()
}