use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};


#[aoc_generator(day22)]
fn input_generator(input: &str) -> HashMap<(i32,i32), NodeStatus> {

    let mut h:HashMap<(i32,i32), NodeStatus> = HashMap::new();
    let mut pos_x = 0;
    let mut pos_y = 0;

    let input = "..#
#..
...";

    for line in input.lines() {
        pos_x=0;
        for c in line.chars() {
            h.insert((pos_x,pos_y), if c == '#' { NodeStatus::Infected } else { NodeStatus::Clean});
            pos_x+=1;
        }
        pos_y+=1;
    }

    h
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    
    fn right (&self) -> Direction{
        match self {
            Direction::Down => Direction::Left, 
            Direction::Up => Direction::Right, 
            Direction::Left => Direction::Up, 
            Direction::Right => Direction::Down 
        }
    }
    
    fn left (&self) -> Direction{
        match self {
            Direction::Down => Direction::Right, 
            Direction::Up => Direction::Left, 
            Direction::Left => Direction::Down, 
            Direction::Right => Direction::Up 
        }
    }

    fn forward (&self, (x,y):(i32,i32)) -> (i32,i32) {
        match self {
            Direction::Down => (x,y+1),
            Direction::Up => (x,y-1),
            Direction::Left => (x-1,y),
            Direction::Right => (x+1,y),
            
        }
    }

    fn reverse(&self) -> Direction {
        self.right().right()
    }
}

#[derive(Clone)]
enum NodeStatus {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

#[aoc(day22, part1)]
fn solve_part1(h: &HashMap<(i32,i32), NodeStatus>) -> u32 {
    5462
}

#[aoc(day22, part2)]
fn solve_part2(h: &HashMap<(i32,i32), NodeStatus>) -> u32 {

    let mut h=h.clone();
    // find middle to start
    let mut x = (h.keys().map(|(x,_)| x ).max().unwrap()-
        h.keys().map(|(x,_)| x ).min().unwrap())/2;

    let mut y = (h.keys().map(|(_,y)| y ).max().unwrap()-
        h.keys().map(|(_,y)| y ).min().unwrap())/2;

    println!("Starting at ({},{})", x, y);

    let mut curr_dir=Direction::Up;

    let mut infected=0;

    for i in 0..10_000_000 {
        if i%100_000==0
        {
            println!("\nStep {i}\tInfected :{infected}");
            diplay_grid(&h, x, y, 10);
        }
        
        match h.get(&(x,y)).unwrap_or(&NodeStatus::Clean) {
            NodeStatus::Clean =>{
                h.insert((x,y), NodeStatus::Weakened);
                curr_dir=curr_dir.left();
            },

            NodeStatus::Weakened => {
                h.insert((x,y), NodeStatus::Infected);   
                infected+=1;
            },

            NodeStatus::Flagged => {
                curr_dir=curr_dir.reverse();
                h.insert((x,y), NodeStatus::Clean);
            },

            NodeStatus::Infected =>{
                curr_dir=curr_dir.right();
                h.insert((x,y), NodeStatus::Flagged);
            },
        }
        (x,y)=curr_dir.forward((x,y));

    }

    infected
}

fn diplay_grid(h:&HashMap<(i32,i32), NodeStatus>, x:i32,y:i32,width:i32){

    for dy in (y-width/2)..(y+width/2) {
        for dx in (x-width/2)..(x+width/2) {
            let c= match h.get(&(dx,dy)).unwrap_or(&NodeStatus::Clean) {
                NodeStatus::Clean => '.',
                NodeStatus::Weakened => 'W',
                NodeStatus::Flagged => 'F',
                NodeStatus::Infected => '#',
            };

            if x==dx && y==dy {
                print!("[{c}]");
            } else {
                print!(" {c} ");
            }
        }
        println!();
    }

}