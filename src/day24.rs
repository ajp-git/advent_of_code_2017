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

    let res=connect(0, input);

    println!("{:?}", res);
    res.0
}

#[aoc(day24, part2)]
fn solve_part2(input:&Vec<(u8,u8)>) -> u32 {

    let res=connect(0, input);

    println!("{:?}", res);
    res.1
}

/// returns (weight, length) 
fn connect(start: u8, avail: &Vec<(u8,u8)>) -> (u32, u32) {

    let mut weight:u32=0;
    let mut length:u32=0;

    if let Some((pos, next)) = avail.iter().enumerate().find(|(pos,(a,b))|a==&start||b==&start) {
        
        let mut shrink=avail.clone();
        shrink.remove(pos);

        weight+=(next.0+next.1) as u32;
        length+=1;

        let res;
        if next.0 ==start {
            res = connect(next.1, &shrink);
        } else {
            res = connect(next.0, &shrink);
        }
        return (res.0+weight,res.1+length);
    } else {
        return (weight,length)
    }
}
