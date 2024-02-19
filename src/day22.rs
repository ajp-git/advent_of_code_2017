use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};


#[aoc_generator(day22)]
fn input_generator(input: &str) -> HashMap<(i32,i32), bool> {

    let mut h:HashMap<(i32,i32), bool> = HashMap::new();
    let mut pos_x = 0;
    let mut pos_y = 0;

    let input = "..#
#..
...";

    for line in input.lines() {
        pos_x=0;
        for c in line.chars() {
            h.insert((pos_x,pos_y), if c == '#' { true } else { false});
            pos_x+=1;
        }
        pos_y+=1;
    }

    println!("{:?}", h);
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
}

#[aoc(day22, part1)]
fn solve_part1(h: &HashMap<(i32,i32), bool>) -> u32 {

    let mut h=h.clone();
    // find middle to start
    let mut x = (h.keys().map(|(x,_)| x ).max().unwrap()-
        h.keys().map(|(x,_)| x ).min().unwrap())/2;

    let mut y = (h.keys().map(|(_,y)| y ).max().unwrap()-
        h.keys().map(|(_,y)| y ).min().unwrap())/2;

    println!("Starting at ({},{})", x, y);

    let mut curr_dir=Direction::Up;

    let mut infected=0;

    for i in 0..10000 {
        match h.get(&(x,y)).unwrap_or(&false) {
            false =>{
                curr_dir=curr_dir.left();
                h.insert((x,y), true);
                infected+=1;
            },
            true =>{
                curr_dir=curr_dir.right();
                h.insert((x,y), false);
            },
        }
        (x,y)=curr_dir.forward((x,y));
        if i%500==0{
            println!("\nStep {i}");
            diplay_grid(&h, x, y, 10);
        }
    }

    infected
}

fn diplay_grid(h:&HashMap<(i32,i32), bool>, x:i32,y:i32,width:i32){

    for dy in (y-width/2)..(y+width/2) {
        for dx in (x-width/2)..(x+width/2) {
            if x==dx && y==dy {
                print!("{}",if h.get(&(dx,dy)).unwrap_or(&false) == &true {"[#]"} else{"[.]"});
            } else {
                print!("{}",if h.get(&(dx,dy)).unwrap_or(&false) == &true {" # "} else{" . "})                
            }
        }
        println!();
    }

}