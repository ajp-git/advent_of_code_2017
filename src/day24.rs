use aoc_runner_derive::{aoc, aoc_generator};


#[aoc_generator(day24)]
fn input_generator(input: &str) -> Vec<(u8,u8)> {

    let mut v:Vec<(u8,u8)>=Vec::new();

    for line in input.lines() {
        let mut sp=line.split('/');
        v.push((
            sp.next().unwrap().parse::<u8>().unwrap(),
            sp.next().unwrap().parse::<u8>().unwrap()
        ))
    }

    println!("{:?}",v);
    v
}

#[aoc(day24, part1)]
fn solve_part1(input:&Vec<(u8,u8)>) -> u32 {

    let res=connect1(0, input, 0, 0);

    println!("{:?}", res);
    res.0
}

#[aoc(day24, part2)]
fn solve_part2(input:&Vec<(u8,u8)>) -> u32 {

    let res=connect2(0, input, 0, 0);

    println!("{:?}", res);
    res.1
}

/// returns (weight, length) 
fn connect1(start: u8, avail: &Vec<(u8,u8)>, length:u32, weight:u32 ) -> (u32, u32) {

    if avail.is_empty() {
        return (weight,length);
    }
    
    let mut max_weight=0;
    let mut max_length=0;

    for (pos, next) in avail.iter().enumerate() {
        
        if next.0 == start || next.1 == start {
            let mut shrink=avail.clone();
            let s = if next.0==start { next.1} else { next.0};
            shrink.remove(pos);
            
            let res = connect1(s, &shrink, length+1, (next.0+next.1) as u32);
            max_weight=max_weight.max(res.0);
            max_length=max_length.max(res.1);
        }
    }
    (weight+max_weight,length+max_length)
}


/// returns (weight, length) 
fn connect2(start: u8, avail: &Vec<(u8,u8)>, length:u32, weight:u32 ) -> (u32, u32) {

    if avail.is_empty() {
        return (weight,length);
    }
    
    let mut max_weight=0;
    let mut max_length=0;

    for (pos, next) in avail.iter().enumerate() {
        
        if next.0 == start || next.1 == start {
            let mut shrink=avail.clone();
            let s = if next.0==start { next.1} else { next.0};
            shrink.remove(pos);
            
            let res = connect2(s, &shrink, length+1, (next.0+next.1) as u32);
            max_weight=max_weight.max(res.0);
            max_length=max_length.max(res.1);
        }
    }
    (weight+max_weight,length+max_length)
}
