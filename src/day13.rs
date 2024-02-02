use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug,Clone)]
enum Direction {
    Up,
    Down,
}

#[derive(Debug,Clone)]
struct Layer{
    index:u32,
    levels: u32,
    current_level:u32,
    direction:Direction,
}

impl Layer {
    fn next_step(&mut self) {
        match self.direction {
            Direction::Down => {
                if self.current_level<self.levels-1 {
                    self.current_level+=1;
                    
                }else {
                    self.direction=Direction::Up;
                    self.current_level-=1;
                }
            },
            Direction::Up => {
                if self.current_level>0 {
                    self.current_level-=1;
                    
                }else {
                    self.direction=Direction::Down;
                    self.current_level+=1;
                }
            },        
        }
    }

    fn is_top (&self) -> bool {
        self.current_level==0
    }
}

#[aoc_generator(day13)]
fn input_generator(input: &str) -> Vec<Layer> {
//    let input="0: 3\n1: 2\n4: 4\n6: 4";
    let mut v:Vec<Layer>=Vec::new();
    for line in input.lines(){
        let l=line.split(": ").collect::<Vec<&str>>();
        v.push(Layer { 
            index: l[0].parse::<u32>().unwrap(), 
            levels: l[1].parse::<u32>().unwrap(),
            current_level: 0,
            direction: Direction::Down });
    }
    v
}

#[aoc(day13, part1)]
fn solve_part1(input: &Vec<Layer>) -> usize {

    let mut input = input.clone();
    let mut total:u32=0;
    let right=input.iter().map(|f|f.index).max().unwrap();

    for i in 0..=right{
        println!("\nIteration : {i}");
        let s_left=if i<8 {0}else{i-8};
        let s_right=(i+8).min(right-1);
        display_firewall(&input,s_left, s_right,i);
        if let Some(t)=input.iter().find(|f|f.index==i){
            if t.is_top(){
                total+=t.levels*t.index;
            }

        }
        for f in input.iter_mut(){
            f.next_step();
        }
    }
    total as usize
}


#[aoc(day13, part2)]
fn solve_part2(input: &Vec<Layer>) -> usize {

    let mut delay:u32=0;
    let mut min_total: u32=u32::MAX;
    loop {
        let mut input = input.clone();
        let mut total:u32=0;

        let right=input.iter().map(|f|f.index).max().unwrap();
    
        for _ in 0..=delay {
            for f in input.iter_mut(){
                f.next_step();
            }
        }
        for i in 0..=right{
            if let Some(t)=input.iter().find(|f|f.index==i){
                if t.is_top(){
                    total+=t.levels*t.index;
                }
    
            }
            for f in input.iter_mut(){
                f.next_step();
            }
        }
        if total < min_total{
            min_total=total;
            println!("New min {} for delay {}\n", min_total, delay);
        }
        print!("\rDelay : {} \tTotal : {}", delay, total);
        if total==0{
            break;
        }else {
            delay+=1;
        }            
    }
    delay as usize
}


fn display_firewall(input:&Vec<Layer>, left:u32, right:u32,current_layer:u32){

    let max_levels=input.iter().map(|f| f.levels).max().unwrap();
    for i in 0..=input.last().unwrap().index {
        if i >=left && i <= right{
            print!("{:>3} ",i);
        }
    }
    println!();

//    for j in 0..max_levels {
    for j in 0..5 {
            for i in 0..=input.last().unwrap().index {
            if i >=left && i <= right{
                if let Some(t)=input.iter().find(|f|f.index==i) {
                    if j==0 && current_layer==t.index {
                        print!(" (S)");
                    }else if t.current_level==j{
                        print!(" [S]");
                    }else if t.levels>j{
                        if j==0 && current_layer==i {
                            print!(" ( )");                    
                        }else {
                            print!(" [ ]");                                            
                        }
                    } else {
                        print!("    ");
                    }
                } else {
                    if j==0 && current_layer==i {
                        print!(" (.)");
                    }else {
                        print!(" ...");                    
                    }
                }
            }
        }
        println!();
    }
    println!();
}