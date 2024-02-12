use std::cmp::{max, min};
use crossterm::{execute, terminal, ExecutableCommand};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day19)]
fn input_generator(input: &str) -> Vec<Vec<char>> {
    let mut v:Vec<Vec<char>>=Vec::new();

//    let input="s1,x3/4,pe/b";

    for line in input.lines() {
        let l_c = line.chars().collect::<Vec<char>>();
        v.push(l_c);
    }
    println!("---Maze loaded");
    v
}
enum Direction{
    Nord,
    East,
    South,
    West,
}

/*struct Packet {
    dir:Direction,
    track:String,
    pos:(u32, u32),
}
struct Network{
    packet:Packet,
    grid:Vec<Vec<char>>,
}
*/
#[derive(Debug, PartialEq, Clone, Copy)]
struct Pos{
    x:i32,
    y:i32,
}

struct Maze {
    grid:Vec<Vec<char>>,
    pos_in:Option<Pos>,
    pos_out:Option<Pos>,
    min_path_len:Option<u32>,
    pos_current:Pos,
    current_dir:Direction,
}

impl Maze {
    pub fn new (grid:&Vec<Vec<char>>) -> Self {
        Maze{
            grid:grid.clone(),
            min_path_len:None,
            pos_current:Pos{x:0, y:0},
            current_dir:Direction::South,
            pos_in:None,
            pos_out:None
        }
    }

    pub fn get_entry(&mut self) -> Option<Pos> {
        for (x, &c) in self.grid[0].iter().enumerate(){
            if c =='|' {
                self.set_entry(Pos{x:x as i32, y:0});
                return self.pos_in;
            }
        }
        None
    }
    pub fn set_entry(&mut self, pos:Pos){
        self.pos_in=Some(pos);
    }

}

impl Maze {
    fn display_maze(&self, current_pos:&Pos, width:i32) {

        let band=width/2;
        
        let max_x=current_pos.x+band;
        let max_y=current_pos.y+band;
        let min_x=current_pos.x-band;
        let min_y=current_pos.y-band;
    
        let min_x=max(0,min_x) as usize;
        let min_y=max(0, min_y) as usize;
        let max_x=min(max_x,self.grid[0].len() as i32-1) as usize;
        let max_y=min(max_y,self.grid.len() as i32-1) as usize;
        
        for pos_y in min_y..max_y {
            for pos_x in min_x..max_x {
                let p_pos=Pos{x:pos_x as i32, y:pos_y as i32};
                if &p_pos == current_pos {
                    // Red background for current position
                    print!("\x1B[41m");
                } else {
                    match self.grid[pos_y as usize][pos_x as usize] {
                        '1' => print!("\x1B[47m \x1B[0m "), // White background for walls
                        _ => print!(""), // Normal output for paths
                    }
                }

                let c:char = self.grid[pos_y as usize][pos_x as usize];
                //print!("{c} :{},{}", pos_x,pos_y);
                print!("{c}");
                if &p_pos == current_pos {
                    // Red background for current position
                    print!("\x1B[0m ");
                }
            }
            println!();
        }
    }
}
fn get_move(v:&Vec<Vec<char>>, pos:Pos,dir:Direction)->Option<Pos> {
    let size_x=v[0].len();
    let size_y=v.len();

    match dir {
        Direction::South =>{
            if pos.y+1==size_y as i32{
                return None;
            }
            Some(Pos{x:pos.x,y:pos.y+1})
        },
        Direction::Nord => {
            if pos.y-1<0 as i32{
                return None;
            }
            Some(Pos{x:pos.x,y:pos.y-1})
        }
        ,
        Direction::East => {
            if pos.x+1>size_y as i32{
                return None;
            }
            Some(Pos{x:pos.x+1,y:pos.y})
        },
        Direction::West => {
            if pos.x-1<0 as i32{
                return None;
            }
            Some(Pos{x:pos.x-1,y:pos.y})
        }
    }
}

#[aoc(day19, part1)]
fn solve_part1(input: &Vec<Vec<char>>) -> String {

    let mut maze:Maze=Maze::new(input);

    if maze.get_entry().is_none() {
        println!("No entry found");
        return "".to_string();
    }
    let mut pkt_pos=maze.pos_in.unwrap();

    loop {
        maze.display_maze(&pkt_pos, 12);
        match input[pkt_pos.y as usize][pkt_pos.y as usize] {
           _ => panic!("input not handled : {:?}\t at pos {:?}",input[pkt_pos.y as usize][pkt_pos.x as usize], pkt_pos ),
        }

        break;
    }
    "".to_string()
}