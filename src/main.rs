use std::fs::File;
use std::io::prelude::*;
use std::path::Path;



fn main() {
    let input_path = Path::new("input.txt");
    let mut file = File::open(&input_path).unwrap();
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();
    let lines: Vec<&str> = file_contents.split_ascii_whitespace().collect();
    let mut total_increases: u32 = 0;
    for i in 3..lines.len()  {
        let a: u32 = lines[i - 3].parse().unwrap();
        let b: u32 = lines[i - 2].parse().unwrap();
        let c: u32 = lines[i - 1].parse().unwrap();
        let d: u32 = lines[i].parse().unwrap();
        let prev = a + b + c;
        let curr = b + c + d;
        if curr > prev {
            total_increases += 1;
        }
    }
    println!("{}", total_increases);
}
