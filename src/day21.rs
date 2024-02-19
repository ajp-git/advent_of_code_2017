use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Pattern
{
    src:Vec<Vec<char>>,
    alt:Vec<Vec<Vec<char>>>,
    dst:Vec<Vec<char>>,
}

impl Pattern {
    fn new (s: &str) -> Self {
        let mut src:Vec<Vec<char>>=Vec::new();
        let mut dst:Vec<Vec<char>>=Vec::new();
        let mut alt:Vec<Vec<Vec<char>>>=Vec::new();
        println!("Pattern::new(\"{}\"",s);
        let mut s_s = s.split(" => ");
        s_s.next().unwrap().split('/').for_each(|f|src.push(f.chars().collect()));
        s_s.next().unwrap().split('/').for_each(|f|dst.push(f.chars().collect()));
        Pattern{src, dst, alt}
    }

    // angle is 1/4 rotate
    fn rotate (&self, angle:u32) -> Vec<Vec<char>> {

        let mut v:Vec<Vec<char>>=Vec::new();

        match angle {
            0 => {
                v=self.src.clone();
            },
            1 => {
                    for x in 0..self.src[0].len() {
                        let mut tmp:Vec<char>=Vec::new();
                        for y in (0..self.src.len()).rev() {
                            tmp.push(self.src[y][x]);
                        }
                        v.push(tmp);
                    }
                },
            2 => {
                for y in (0..self.src.len()).rev() {
                    let mut tmp:Vec<char>=Vec::new();
                    for x in (0..self.src[0].len()).rev() {
                        tmp.push(self.src[y][x]);
                    }
                    v.push(tmp);
                }

            },
            3 => {
                for x in (0..self.src[0].len()).rev() {
                    let mut tmp:Vec<char>=Vec::new();
                    for y in 0..self.src.len() {
                        tmp.push(self.src[y][x]);
                    }
                    v.push(tmp);
                }

            },
            _ => panic!("invalid angle in pattern::rotate()"),
        }

        /*for y in 0..self.src.len(){
            for x in 0..self.src.len(){
                print!("{}",self.src[y][x]);
            }
            print!("\tgives ");
            for x in 0..v.len(){
                print!("{}",v[y][x]);
            }
            println!();
        }
        println!();*/
        v
    }

    fn matches (&self, chk: &Vec<Vec<char>> ) -> Option<&Vec<Vec<char>>> {

        if self.alt.iter().any(|f| f==chk ) {
            return Some(&self.dst);
        } else {
            panic!("{:?} not found", chk);
            
        }
        None
    }
    
    fn v_flip (&self) -> Vec<Vec<char>> {
        let mut flip:Vec<Vec<char>>=Vec::new();
        let s=self.src.len();

        for y in (0..s).rev() {
            let mut yv:Vec<char>=Vec::new();

            for x in 0..s {
                yv.push(self.src[y][x]);    
            }
            flip.push(yv);
        }
        flip
    }

    fn h_flip (&self) -> Vec<Vec<char>> {
        let mut flip:Vec<Vec<char>>=Vec::new();
        let s=self.src.len();

        for y in (0..s){
            let mut yv:Vec<char>=Vec::new();

            for x in (0..s).rev() {
                yv.push(self.src[y][x]);    
            }
            flip.push(yv);
        }
        flip
    }

    fn display(&self){
        for y in 0..self.src.len(){
            for x in 0..self.src[0].len() {
                print!("{}", self.src[y][x]);
            }
            println!();
        }
    }

    fn generate_alternatives (&mut self) {

        for i in 0..4 {
            
            let p = Pattern{src:self.rotate(i),alt:Vec::new(), dst:Vec::new()};
            self.alt.push(p.src.clone());
            self.alt.push(p.v_flip().clone());
            self.alt.push(p.h_flip().clone());

        }
    }

}

struct  Grid {
    v:Vec<Vec<Vec<char>>>,
    pat:Vec<Pattern>,
}

impl Grid {
    fn new(input: &Vec<Pattern>) -> Self {
        
        Grid {
            v:vec![vec![vec!['.','#','.'],vec!['.','.','#'], vec!['#','#','#']]],
            pat:input.iter().cloned().collect::<Vec<Pattern>>(),
        }
    }

    fn run (&mut self, iterations:u32) {
        self.display();
        for i in 0..iterations {
            let mut o:Vec<Vec<Vec<char>>>=Vec::new();
            for k in 0..self.v.len() {        
                for j in self.pat.iter() {
                    if let Some(r) = j.matches(&self.v[k]) {
                        //println!("Found match for {:?} becomes {:?}", &self.v, r);
                        o.push(r.clone());
                        break;
                    }
                }
            }
            self.v=o.clone();
            self.split();
        }
    }

    fn split(&mut self) {
        if self.v[0].len()%4==0 {

            let mut out:Vec<Vec<Vec<char>>>=Vec::new();
            for z in 0..self.v.len(){
                for x in (0..=self.v[0].len()/2).step_by(2) {
                    let mut oo:Vec<Vec<char>>=Vec::new();
                    for y in (0..=self.v[0].len()/2).step_by(2) {
                        let mut o:Vec<char> = Vec::new();
                        o.push(self.v[z][y][x]);
                        o.push(self.v[z][y][x+1]);
                        oo.push(o.clone());
                        o.clear();
                        o.push(self.v[z][y+1][x]);
                        o.push(self.v[z][y+1][x+1]);   
                        oo.push(o.clone());

//                        println!("oo {oo:?}");
                        out.push(oo.clone());
                        oo.clear();
                    }
                }
            }
            self.v=out.clone();
        }
        self.display();
    }

    fn count_lit(&self) -> usize {
        
        let mut tot:usize=0;

        for r in self.v.iter() {
            for s in r.iter(){
                tot+=s.iter().filter(|&&y|y=='#').count();
            }
        }

        tot
    }

    fn display(&self){

        println!("Display grid");
        let w=self.v.len();
        for z in 0..self.v.len() {            
            for y in 0..self.v[0].len(){
                for x in 0..self.v[0][0].len(){
                    print!("{}",self.v[z][y][x]);
                }
                println!();
            }
            println!("---")
        }

    }
}

#[aoc_generator(day21)]
fn input_generator(input: &str) -> Vec<Pattern> {
    let mut v:Vec<Pattern>=Vec::new();

//    let input="../.# => ##./#../...
//.#./..#/### => #..#/..../..../#..#";
    let _ = input.lines().into_iter().for_each(|l| v.push(Pattern::new(l)));
    let _ = v.iter_mut().for_each(|f| f.generate_alternatives());
    v
}

#[aoc(day21, part1)]
fn solve_part1(input: &Vec<Pattern>) -> usize {

    let mut grid=Grid::new(input);
    grid.run(5);

    grid.count_lit()
}


// Assuming the Pattern struct and its methods are defined above

#[cfg(test)]
mod tests {
    use super::*; // Bring the Pattern struct and its methods into scope

    #[test]
    fn test_pattern_new() {
        let pattern = Pattern::new("123/456/789 => abc/def/ghi");
        assert_eq!(pattern.src, vec![vec!['1', '2', '3'], vec!['4', '5', '6'], vec!['7', '8', '9']]);
        assert_eq!(pattern.dst, vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f'], vec!['g', 'h', 'i']]);
    }

    #[test]
    fn test_rotate_90() {
        let pattern = Pattern::new("123/456/789 => abc/def/ghi");
        let rotated = pattern.rotate(1);
        let expected = vec![
            vec!['7', '4', '1'],
            vec!['8', '5', '2'],
            vec!['9', '6', '3'],
        ];
        assert_eq!(rotated, expected);
    }

    #[test]
    fn test_rotate_180() {
        let pattern = Pattern::new("123/456/789 => abc/def/ghi");
        let rotated = pattern.rotate(2);
        let expected = vec![
            vec!['9', '8', '7'],
            vec!['6', '5', '4'],
            vec!['3', '2', '1'],
        ];
        assert_eq!(rotated, expected);
    }

    #[test]
    fn test_rotate_270() {
        let pattern = Pattern::new("123/456/789 => abc/def/ghi");
        let rotated = pattern.rotate(3);
        let expected = vec![
            vec!['3', '6', '9'],
            vec!['2', '5', '8'],
            vec!['1', '4', '7'],
        ];
        assert_eq!(rotated, expected);
    }


    #[test]
    fn test_matches_90_rotation() {
        let pattern = Pattern::new("123/456/789 => abc/def/ghi");
        let chk = vec![
            vec!['7', '4', '1'],
            vec!['8', '5', '2'],
            vec!['9', '6', '3'],
        ];
        assert_eq!(pattern.matches(&chk), Some(chk));
    }

    #[test]
    fn test_no_match() {
        let pattern = Pattern::new("123/456/789 => abc/def/ghi");
        let chk = vec![
            vec!['x', 'x', 'x'],
            vec!['x', 'x', 'x'],
            vec!['x', 'x', 'x'],
        ];
        assert_eq!(pattern.matches(&chk), None);
    }

    #[test]
    fn test_h_flip_different_pattern() {
        let pattern = Pattern {
            src: vec![
                vec![' ', '.', '#'],
                vec!['.', '#', '.'],
                vec!['#', '.', '#'],
            ],
            dst: vec![], // The destination is not relevant for this test
        };

        let expected = vec![
            vec!['#', '.', ' '],
            vec!['.', '#', '.'],
            vec!['#', '.', '#'],
        ];

        assert_eq!(pattern.h_flip(), expected);
    }

    #[test]
    fn test_v_flip_different_pattern() {
        let pattern = Pattern {
            src: vec![
                vec![' ', '.', '#'],
                vec!['.', '#', '.'],
                vec!['#', '.', '#'],
            ],
            dst: vec![], // The destination is not relevant for this test
        };

        let expected = vec![
            vec!['#', '.', '#'],
            vec!['.', '#', '.'],
            vec![' ', '.', '#'],
        ];

        assert_eq!(pattern.v_flip(), expected);
    }

    // Additional tests can be written for other rotation angles if implemented
    // For example, a 270-degree rotation (angle 3) and no rotation (angle 0 or 4)

    // ...
}

// The rest of your Rust code (e.g., the definition of the Pattern struct) would go here