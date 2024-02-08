use std::cmp::{max, min};

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
#[derive(Debug)]
struct Pos{
    x:i32,
    y:i32,
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

fn display_maze(maze:&Vec<Vec<char>>, x:i32, y:i32, width:i32) {

    let band=width/2;
    
    let max_x=x+band;
    let max_y=y+band;
    let min_x=x-band;
    let min_y=y-band;

    let min_x=max(0,min_x) as usize;
    let min_y=max(0, min_y) as usize;
    let max_x=min(max_x,maze[0].len() as i32-1) as usize;
    let max_y=min(max_y,maze.len() as i32-1) as usize;
    
    for pos_y in min_y..max_y {
        for pos_x in min_x..max_x {
            let c:char = maze[pos_y as usize][pos_x as usize];
            //print!("{c} :{},{}", pos_x,pos_y);
            print!("{c}");
        }
        println!();
    }
}

#[aoc(day19, part1)]
fn solve_part1(input: &Vec<Vec<char>>) -> String {

    let mut entry=0;
    for (x, &c) in input[0].iter().enumerate(){
        if c =='|' {
            entry=x as i32;
            println!("Found entry : {:?}", entry);
            break;
        }
    }
    let mut pkt_pos=Pos{x:entry, y:0};
    let mut pkt_dir:Direction=Direction::South;

    loop {
        display_maze(&input, pkt_pos.x, pkt_pos.y, 30);
        match input[pkt_pos.y as usize][pkt_pos.y as usize] {
           _ => panic!("input not handled : {:?}\t at pos {:?}",input[pkt_pos.x as usize][pkt_pos.y as usize], pkt_pos ),
        }

        break;
    }
    "".to_string()
}