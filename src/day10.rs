use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<u8> {
//    input.split(',').filter_map(|f|f.trim().parse::<usize>().ok()).collect::<Vec<usize>>()
    input.chars().map(|c|c as u8).collect::<Vec<u8>>()
}
#[derive(Debug,Clone)]
struct Ring<T> {
    data: Vec<T>,
    index: usize,
}

impl<T: std::clone::Clone + std::fmt::Display> Ring<T> {

    fn new () -> Self {
        Ring { data: Vec::new(), index: 0 }
    }

    fn display(&self) {
        for i in 0..self.data.len(){
            if let Some(t)=self.get(i) {
                if self.index==i{
                    print!("[{}]",t);
                } else {
                    print!(" {} ",t);
                }    
            }
        }
        println!();
    }
    fn first (&self)-> Option<&T> {
        self.data.get(0)
    }

    fn current(&self) -> usize {
        self.index
    }

    fn set_current(&mut self, index:usize) {
        self.index=index%self.data.len();
    }

    fn next(&mut self) {
        self.index=(self.index+1)%self.data.len();
    }
    fn previous(&mut self) {
        self.index=if self.index==0 {self.data.len()-1} else {self.index-1};
    }

    fn get(&self, index:usize) -> Option<&T> {
        self.data.get(index%self.data.len())
    }

    fn put(&mut self, index:usize, t:T) {
        let len=self.data.len();
        self.data[index%len]=t;
    }

    fn remove(&mut self, index:usize) -> Option<T>{
    
        if self.data.is_empty(){
            return None;
        }
        Some(self.data.remove(index%self.data.len()))
    }

    fn push(&mut self, t:T){
        self.data.push(t);
    }
    
    fn insert(&mut self, index:usize, t:T) {
        self.data.insert(index%self.data.len(), t);
    }

    fn rotate_length_from(&mut self, length:usize, from: usize) {
            
        if let Some(t) = self.remove(from+length) {
            self.insert(from, t );
        }
    }

    fn replace_pos(&mut self, pos1: usize, pos2: usize) {
        // Check if the positions are the same or if the ring is empty
        if pos1 == pos2 || self.data.is_empty() {
            return;
        }

        // Use the get method to retrieve the elements at pos1 and pos2
        if let Some(t1) = self.get(pos1).cloned() {
            if let Some(t2) = self.get(pos2).cloned() {
                // Use the put method to swap the elements
                self.put(pos1, t2);
                self.put(pos2, t1);
            }
        }
    }

    fn rev(&mut self, from:usize, length:usize) {
        for i in 0..length/2 {
            self.replace_pos(from+i, from+length-i-1 );
        }
    }


}
#[aoc(day10, part2)]
fn solve_part2(input: &Vec<u8>) -> String {
    let mut input=input.clone();
    /*let mut input:Vec<u8>="1,2,3".to_string().chars().map(|c|c as u8).collect::<Vec<u8>>();  
    let mut input:Vec<u8>="1,2,4".to_string().chars().map(|c|c as u8).collect::<Vec<u8>>();  
    let mut input:Vec<u8>="".to_string().chars().map(|c|c as u8).collect::<Vec<u8>>();  */

    let add:Vec<u8>=vec![17, 31, 73, 47, 23];
    for i in add {
        input.push(i);
    }
    println!("{:?}", input);

    let mut ring:Ring<u32>=Ring::new();
    for i in 0..=255 {
        ring.push(i);
    }

    let mut skip:usize=0;
    for r in 0..64{
        for i in 0..input.len(){
            ring.rev(ring.current(), input[i as usize] as usize);
            ring.set_current(ring.current()+input[i as usize] as usize+skip);
            skip+=1;
        }    
    }
    // Xor calc
    let mut x:Vec<u8>=vec![0;16];
    for i in 0..=255{
        x[i/16]=x[i/16] ^ (*ring.get(i).unwrap() as u8);
    }

    let mut out:String=String::new();
    for i in x.iter(){
        let hex = format!("{:02x}", i);
        out.push_str(&hex);
    }
    println!();
    out
}

#[aoc(day10, part1)]
fn solve_part1(input: &Vec<u8>) -> u32 {
    let input=vec![192,69,168,160,78,1,166,28,0,83,198,2,254,255,41,12];

    let mut ring:Ring<u32>=Ring::new();
    for i in 0..=255 {
        ring.push(i);
    }

    let mut skip:usize=0;
    for i in 0..input.len(){
        ring.rev(ring.current(), input[i] as usize);
        ring.set_current(ring.current()+input[i] as usize+skip);
        skip+=1;
    }
    ring.data[0]*ring.data[1]
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_first() {
        let mut ring = Ring::new();
        assert_eq!(ring.first(), None);
        ring.push(1);
        assert_eq!(ring.first(), Some(&1));
    }

    #[test]
    fn test_next_and_previous() {
        let mut ring = Ring::new();
        ring.push(1);
        ring.push(2);
        ring.push(3);
        assert_eq!(ring.current(), Some(&1));
        ring.next();
        assert_eq!(ring.current(), Some(&2));
        ring.next();
        assert_eq!(ring.current(), Some(&3));
        ring.next();
        assert_eq!(ring.current(), Some(&1));
        ring.previous();
        assert_eq!(ring.current(), Some(&3));
    }

    #[test]
    fn test_get_and_put() {
        let mut ring = Ring::new();
        ring.push(1);
        ring.push(2);
        ring.push(3);
        assert_eq!(ring.get(1), Some(&2));
        ring.put(1, 4);
        assert_eq!(ring.get(1), Some(&4));
    }

    #[test]
     fn test_remove() {
        let mut ring = Ring::new();
        ring.push(1);
        ring.push(2);
        ring.push(3);
        assert_eq!(ring.remove(1), Some(2)); 
        assert_eq!(ring.data, vec![1, 3]);
        assert_eq!(ring.remove(1), Some(3)); 
        assert_eq!(ring.data, vec![1]);
    }
    

    #[test]
    fn test_insert() {
        let mut ring = Ring::new();
        ring.push(1);
        ring.push(3);
        ring.insert(1, 2);
        assert_eq!(ring.data, vec![1, 2, 3]);
    }

    #[test]
    fn test_rotate_length_from() {
        let mut ring = Ring::new();
        for i in 0..5 {
            ring.push(i);
        }
        ring.rotate_length_from(2, 1);
        assert_eq!(ring.data, vec![0, 3, 1, 2, 4]);
    }

    #[test]
    fn test_replace_pos() {
        let mut ring = Ring::new();
        ring.push(1);
        ring.push(2);
        ring.replace_pos(0, 1);
        assert_eq!(ring.data, vec![2, 1]);
    }

    #[test]
    fn test_rev() {
        let mut ring = Ring::new();
        for i in 0..5 {
            ring.push(i);
        }
        ring.rev(1, 3);
        assert_eq!(ring.data, vec![0, 3, 2, 1, 4]);
    }
}
