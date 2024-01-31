use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day9)]
fn input_generator(input: &str) -> String {
    input.to_string()
}


#[aoc(day9, part1)]
fn solve_part1(input: &String) -> u32 {

//    let input="{{<ab>},{<ab>},{<ab>},{<ab>}}".to_string();
//    let input="{{<!!>},{<!!>},{<!!>},{<!!>}}".to_string();
//    let input="{{<a!>},{<a!>},{<a!>},{<ab>}}".to_string();
//    let input="{{{},{},{{}}}}".to_string();
    
    let mut in_garbage=false;
    let mut ignore_next_char=false;
    let mut level:u32=0;
    let mut total=0;

    for c in input.chars(){
        if ignore_next_char { ignore_next_char=false; continue;}
        if c=='!' { ignore_next_char=true; continue;}
        if in_garbage {
            if c=='>' {
                in_garbage=false;
            }
            continue;
        }
        match c {
            '<' => in_garbage=true,
            '!' => ignore_next_char=true,
            '{' => {
                level+=1;
                total+=level;
            },
            '}' => level-=1,
            _ => {},
        }
    }
    total as u32
}

#[aoc(day9, part2)]
fn solve_part2(input: &String) -> u32 {

//    let input="{{<ab>},{<ab>},{<ab>},{<ab>}}".to_string();
//    let input="{{<!!>},{<!!>},{<!!>},{<!!>}}".to_string();
//    let input="{{<a!>},{<a!>},{<a!>},{<ab>}}".to_string();
//    let input="{{{},{},{{}}}}".to_string();
    
    let mut in_garbage=false;
    let mut ignore_next_char=false;
    let mut level:u32=0;
    let mut total=0;

    for c in input.chars(){
        if ignore_next_char { ignore_next_char=false; continue;}
        if c=='!' { ignore_next_char=true; continue;}
        if in_garbage {

            if c=='>' {
                in_garbage=false;
            } else {
                total+=1;
            }
            continue;
        }
        match c {
            '<' => in_garbage=true,
            '!' => ignore_next_char=true,
            _ => {},
        }
    }
    total as u32
}