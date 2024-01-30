use std::{collections::{HashMap, HashSet}};
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug, Clone)]
struct Disk {
    weight:Option<u32>,
    children:Vec<String>,
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> HashMap<String, Disk> {

    /*let input="pbga (66)
    xhth (57)
    cntj (57)
    ktlj (57)
    fwft (72) -> ktlj, cntj, xhth
    ebii (61)
    havc (66)
    qoyq (66)
    padx (45) -> pbga, havc, qoyq
    tknk (41) -> ugml, padx, fwft
    jptl (61)
    ugml (68) -> gyxo, ebii, jptl
    gyxo (61)";*/

    let mut v:HashMap<String,Disk>=HashMap::new();
    let re_disk=Regex::new(r"^(\w+)\s\((\d+)\)(?:\s->\s(.*))?$").unwrap();

    for line in input.lines() {
        let caps=re_disk.captures(line.trim_start()).unwrap();
        let name=caps.get(1).unwrap().as_str().to_string();
        if v.get(&name).is_none() {
            v.insert(name.clone(), 
            Disk { 
                weight: Some(caps.get(2).unwrap().as_str().parse::<u32>().unwrap()),
                children: Vec::new() 
            });
        } else if caps.get(2).is_some() {
            let mut disk=v.get_mut(&name).unwrap();
            disk.weight=Some(caps.get(2).unwrap().as_str().parse::<u32>().unwrap());
        }
        if let Some(children)=caps.get(3) {
            let children=children.as_str().split(", ");
            for child in children {
                if v.get(child).is_none() {
                    v.insert(child.to_string(), Disk{weight: None, children:Vec::new()});
                } 
                let current: &mut Disk=v.get_mut(&name).unwrap();
                current.children.push(child.to_string())
            }
        }
    }
//   println!("{:?}", v);
    v
}


#[aoc(day7, part1)]
fn solve_part1(input: &HashMap<String, Disk>) -> Option<String> {

    let mut children:HashSet<String>=HashSet::new();
    for (_, &ref d) in input {
        for child in d.children.iter(){
            children.insert(child.to_string());
        }
    }
    for (&ref k, _) in input {
        if ! children.contains(k) {
            return Some(k.clone());
        }
    }

    None
}
fn get_weight(input: &HashMap<String,Disk>, name:&String) -> u32{

    let current:&Disk=input.get(name).unwrap();
    let mut total=current.weight.unwrap();
    let mut v:Vec<(String,u32)>=Vec::new();

    for k in &current.children {
        let s_weight= get_weight(&input, &k);
        v.push((k.clone(),s_weight));
//        println!("Getting weights of {k} children : {}", s_weight);
        total+=s_weight;
    }

    if v.len()>0 {
        let theorical_any=v.iter().map(|(_,v)| v ).sum::<u32>()/v.len() as u32;
        //println!("Found for {} : {:?}",name, v);
        if v[0].1 != theorical_any {
            println!("\n\nFound unbalanced ! for {} : {:?}",name, v);
            let v_min=v.iter().map(|(_,v)|v).min().unwrap();
            let v_max=v.iter().map(|(_,v)|v).max().unwrap();
            println!("min:{v_min}\tv_max:{v_max}\t{name}.weight={}",current.weight.unwrap());
            for (k,v) in v.iter(){
                let r=input.get(k).unwrap();
                println!("\t{} w : {}", k, r.weight.unwrap());
            }
        }
    } 
    total
}

#[aoc(day7, part2)]
fn solve_part2(input: &HashMap<String, Disk>) -> u32 {

    get_weight(input, &"vvsvez".to_string());
//    get_weight(input, &"tknk".to_string());
    0
}
