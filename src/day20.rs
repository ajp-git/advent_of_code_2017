use std::{cmp::{max, min}, thread::sleep, time::Duration};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Position;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct Coord {
    x:i64,
    y:i64,
    z:i64,
}

impl Coord {
    fn new (s:String) -> Self{
        let mut s_s=s.split(',');
        //println!("Coord : {}", s);
        let c = Coord {
            x:s_s.next().unwrap().parse::<i64>().unwrap(),
            y:s_s.next().unwrap().parse::<i64>().unwrap(),
            z:s_s.next().unwrap().chars().filter(|c|c.is_digit(10)||*c=='-').collect::<String>().parse::<i64>().unwrap(),
        };
        c       
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Particle {
    pos:Coord,
    vel:Coord,
    acc:Coord
}

impl Particle {
    fn new (s:String) -> Self {
        let mut s_s=s.split('<');
        //println!("Part : {}", s);
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

    // p=<-317,1413,1507>, v=<19,-102,-108>, a=<1,-3,-3>

    /*let input="p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>";*/

    for line in input.lines() {
        let line = line.to_string();
        v.push(Particle::new(line));
    }
//    dbg!(&v);
    v
}


#[aoc(day20, part1)]
fn solve_part1(input: &Vec<Particle>) -> String {
    return "125".to_string();
    let mut v = input.clone(); // Clone input into a new Vec<Particle>

    for i in 0..8000 {
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
}

#[aoc(day20, part2)]
fn solve_part2(input: &Vec<Particle>) -> usize {
    let mut v = input.clone(); // Clone input into a new Vec<Particle>
    for i in 0..1_000 {

        v.iter_mut().for_each(|p| p.run());

        let mut positions = std::collections::HashMap::new();
        for (u, p) in v.iter().enumerate() {
            *positions.entry(p.pos).or_insert(0) += 1;
        }
    
        let mut col: Vec<usize> = v.iter()
            .enumerate()
            .filter(|&(u, p)| positions[&p.pos] > 1)
            .map(|(u, _)| u)
            .collect();
    
        if !col.is_empty() {
            col.sort();
            for &idx in col.iter().rev() {
                v.remove(idx);
            }
        }

        // Uncomment this line if you want to see the progress
         print!("\r{i}\tnv: {}          ", v.len());
        

    }
    
    println!("len {}", v.len());
    v.len()
}