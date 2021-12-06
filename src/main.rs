use std::fs::File;
use std::io::{self, BufRead, Error};

use std::path::Path;


fn main() -> Result<(), Error> {
    let input_path = Path::new("input.txt");
    let file = File::open(&input_path).unwrap();
    let file_reader = io::BufReader::new(file);

    let mut horizontal_position: u32 = 0;
    let mut depth: u32 = 0;
    let mut aim: u32 = 0;

    for line in file_reader.lines() {
        if let Ok(direction_scalar) = line {
            let split: Vec<&str> = direction_scalar.split(' ').collect();
            match split[0] {
                "forward" => {
                    let x = split[1].parse::<u32>().unwrap();
                    horizontal_position += x;
                    depth += aim * x;
                },
                "down" => {
                    aim += split[1].parse::<u32>().unwrap();
                },
                "up" => {
                    aim -= split[1].parse::<u32>().unwrap();
                },
                _ => panic!("Unknown direction")
            }
        }
    }
    println!("{}", horizontal_position * depth);
    Ok(())
}
