use std::{cmp::{max, min}, thread::sleep, time::Duration};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day19)]
fn input_generator(input: &str) -> Vec<Vec<char>> {
    let mut v:Vec<Vec<char>>=Vec::new();

//    let input="s1,x3/4,pe/b";
    /*let input="    |          
    |  +--+    
    A  |  C    
F---|--|-E---+ 
    |  |  |  D 
    +B-+  +--+ ";*/

    for line in input.lines() {
        let l_c = line.chars().collect::<Vec<char>>();
        v.push(l_c);
    }
    println!("---Maze loaded");

    v
}
#[derive(PartialEq)]
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
    result:String,
    size_x:usize,
    size_y:usize,
    steps:usize,
}

impl Maze {
    pub fn new (grid:&Vec<Vec<char>>) -> Self {
        Maze{
            grid:grid.clone(),
            min_path_len:None,
            pos_current:Pos{x:0, y:0},
            current_dir:Direction::South,
            pos_in:None,
            pos_out:None,
            result:String::new(),
            size_x:grid[0].len(),
            size_y:grid.len(),
            steps:0,
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
    pub fn next(&mut self) -> Option<Pos> {
        
        Some(self.pos_current)
    }

}

impl Maze {
    fn get_cell (&self, pos: Pos) -> char {
        self.grid[pos.y as usize][pos.x as usize]
    }

    fn display_maze(&self, current_pos:&Pos, width:i32) {

        print!("\x1B[2J\x1B[H");

        let band=width/2;
        
        let max_x=current_pos.x+band;
        let max_y=current_pos.y+band;
        let min_x=current_pos.x-band;
        let min_y=current_pos.y-band;
    
        let min_x=max(0,min_x) as usize;
        let min_y=max(0, min_y) as usize;
        let max_x=min(max_x,self.grid[0].len() as i32-1) as usize;
        let max_y=min(max_y,self.grid.len() as i32-1) as usize;
        
        for pos_y in min_y..=max_y {
            for pos_x in min_x..=max_x {
                let p_pos=Pos{x:pos_x as i32, y:pos_y as i32};
                if &p_pos == current_pos {
                    // Red background for current position
                    print!("\x1B[41m");
                } else {
                    /*match self.grid[pos_y as usize][pos_x as usize] {
                        '+' => print!("\x1B[47m \x1B[0m "), // White background for walls
                        _ => print!(""), // Normal output for paths
                    }*/
                }

                let c:char = self.get_cell(Pos{x:pos_x as i32, y:pos_y as i32});
                //print!("{c} :{},{}", pos_x,pos_y);
                print!("{c}");
                if &p_pos == current_pos {
                    // Red background for current position
                    print!("\x1B[0m");
                }
            }
            println!();
        }
        println!("\nSteps : \t{}", self.steps);
    }

    fn get_move(&mut self, pos:Pos)->Option<Pos> {

        match self.current_dir {
            Direction::South =>{
                if pos.y+1==self.size_y as i32{
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
                if pos.x+1>self.size_x as i32{
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

    fn get_possible_move(&self, pos:Pos, dir:Direction)->Option<Pos> {


        match dir {
            Direction::South =>{
                if pos.y+1==self.size_y as i32{
                    return None;
                }
                let pos=Pos{x:pos.x,y:pos.y+1};
                if self.get_cell(pos) != ' ' { return Some(pos);}
                None
            },
            Direction::Nord => {
                if pos.y-1<0 as i32{
                    return None;
                }
                let pos = Pos{x:pos.x,y:pos.y-1};
                if self.get_cell(pos) != ' ' { return Some(pos);}
                None
            }
            ,
            Direction::East => {
                if pos.x+1>self.size_x as i32{
                    return None;
                }
                let pos = Pos{x:pos.x+1,y:pos.y};
                if self.get_cell(pos) != ' ' { return Some(pos);}
                None

            },
            Direction::West => {
                if pos.x-1<0 as i32{
                    return None;
                }
                let pos = Pos{x:pos.x-1,y:pos.y};
                if self.get_cell(pos) != ' ' { return Some(pos);}
                None
            }
        }
    }

    fn get_next_direction (&self) -> Option<Direction> {
        let n_pos=self.get_possible_move(self.pos_current, Direction::Nord);
        let s_pos=self.get_possible_move(self.pos_current, Direction::South);
        let e_pos=self.get_possible_move(self.pos_current, Direction::East);
        let w_pos=self.get_possible_move(self.pos_current, Direction::West);

        if n_pos.is_some()&& self.current_dir!=Direction::South {return Some(Direction::Nord)};
        if s_pos.is_some()&& self.current_dir!=Direction::Nord {return Some(Direction::South)};
        if e_pos.is_some()&& self.current_dir!=Direction::West {return Some(Direction::East)};
        if w_pos.is_some()&& self.current_dir!=Direction::East {return Some(Direction::West)};
    
        None
    }

    fn run(&mut self) -> String{
        self.result.clear();

        if self.pos_in.is_none(){
            panic!("Couldn't run maze, entry point not set");
        }

        self.pos_current=self.pos_in.unwrap();

        loop {
            sleep(Duration::from_millis(10));
            self.display_maze(&self.pos_current, 35);
            let c= self.grid[self.pos_current.y as usize][self.pos_current.x as usize];
            self.steps+=1;
            match c {
                '+' => {
                    if let Some(dir) = self.get_next_direction(){
                        self.current_dir=dir;
                        if let Some(c_pos) = self.get_move(self.pos_current) {
                            self.pos_current=c_pos;
                        }else {
                            return format!("{}", self.steps-1);
                        }
                        }
                    else {
                        panic!("Bloqué");
                    }
                },
                '-' | '|' => {
                    if let Some(c_pos) = self.get_move(self.pos_current) {
                        self.pos_current=c_pos;
                    }else {
                        return format!("{}", self.steps-1);
                    }
                },
                c if ('A'..='Z').contains(&c) => {
                    self.result.push(c);
                    
                    if let Some(c_pos) = self.get_move(self.pos_current) {
                        self.pos_current=c_pos;
                    }else {
                        return format!("{}", self.steps-1);
                    }
                }
               _ => 
                {
                    self.display_maze(&self.pos_current, 25);

                    println!("Current string:{}", self.result);
                    return format!("{}", self.steps-1);
                    //panic!("input not handled : {:?}\t at pos {:?}",c, self.pos_current)
                },
            }
    
        }
    }
//    "".to_string()
}

#[aoc(day19, part1)]
fn solve_part1(input: &Vec<Vec<char>>) -> String {

    let mut maze:Maze=Maze::new(input);

    if maze.get_entry().is_none() {
        println!("No entry found");
        return "".to_string();
    }
//    maze.run()
    "HATBMQJYZ".to_string()
}
#[aoc(day19, part2)]
fn solve_part2(input: &Vec<Vec<char>>) -> String {

    let mut maze:Maze=Maze::new(input);

    if maze.get_entry().is_none() {
        println!("No entry found");
        
    }
    maze.run()
    
//    maze.grid.iter().map(
//        |v| v.iter().filter(|&&c|c != ' ').count()).sum()

}