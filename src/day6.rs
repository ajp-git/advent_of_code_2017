use std::{cmp::{max, min}, collections::HashSet};
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn input_generator(input: &str) -> Vec<u32> {

    let mut v:Vec<u32>=Vec::new();

    for line in input.lines() {
        let s_line=line.split('\t');
        for (i,s) in s_line.enumerate() {
            println!("{} : {}", i, s);
            v.push((s.parse::<u32>().unwrap()));
        }
    }
    println!("{:?}", v);
    v
}

#[aoc(day6, part1)]
fn solve_part1(input: &Vec<u32>) -> u32 {

    //let input = vec![0,2,7,0];
    let mut h_configs:HashSet<Vec<u32>>=HashSet::new();
    let mut actual=input.clone();
    let mut steps=0;
    let bank_len=actual.len() as u32;
    loop {
        steps+=1;
        h_configs.insert(actual.clone());

        let mut max_blocks=0;
        let mut max_index=0;
        for (i, val) in actual.iter().enumerate() {
            if *val>max_blocks {
                max_blocks=*val;
                max_index=i;
            }
        }
        println!("Bank {} has {} blocks ", max_index, max_blocks);
        println!("before{:?}", &actual);

        let rep_block=max(1,max_blocks/(bank_len-1));
        println!("giving all {} blocks",rep_block);
        actual[max_index]=0;

        for i in max_index+1..(max_index+actual.len()*2) {
            if max_blocks > 0 {
                let rep=min(max_blocks,rep_block);
                actual[(i as u32%bank_len) as usize]+=rep;
                max_blocks-=rep;
            }
        }
        println!("after {:?}\n", &actual);

        if h_configs.get(&actual).is_some(){
            println!("Found !!!");
            break;
        }
        h_configs.insert(actual.clone());
    }
    steps
}


#[aoc(day6, part2)]
fn solve_part2(input: &Vec<u32>) -> u32 {

    //let input = vec![0,2,7,0];
    let mut h_configs:HashSet<Vec<u32>>=HashSet::new();
    let mut actual=input.clone();
    let mut steps=0;
    let bank_len=actual.len() as u32;
    let mut bfound=false;

    loop {
        steps+=1;
        h_configs.insert(actual.clone());

        let mut max_blocks=0;
        let mut max_index=0;
        for (i, val) in actual.iter().enumerate() {
            if *val>max_blocks {
                max_blocks=*val;
                max_index=i;
            }
        }
        println!("Bank {} has {} blocks ", max_index, max_blocks);
        println!("before{:?}", &actual);

        let rep_block=max(1,max_blocks/(bank_len-1));
        println!("giving all {} blocks",rep_block);
        actual[max_index]=0;

        for i in max_index+1..(max_index+actual.len()*2) {
            if max_blocks > 0 {
                let rep=min(max_blocks,rep_block);
                actual[(i as u32%bank_len) as usize]+=rep;
                max_blocks-=rep;
            }
        }
        println!("after {:?}\n", &actual);

        if h_configs.get(&actual).is_some(){
            if bfound{
                println!("Found !!!");
                break;    
            }
            bfound=true;
            h_configs.clear();
            h_configs.insert(actual.clone());
            steps=0;
        }
        h_configs.insert(actual.clone());
    }
    steps
}