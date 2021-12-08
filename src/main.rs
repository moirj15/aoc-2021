use std::fs::File;
use std::io::{prelude::*, self, Error};

use std::path::Path;

fn main() -> Result<(), Error> {
    let input_path = Path::new("input.txt");
    let mut file = File::open(&input_path).unwrap();
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();
    let bit_strings: Vec<&str> = file_contents.split_whitespace().collect();

    let mut one_digit_counts: [u32; 12] = [0; 12];
    let mut zero_digit_counts: [u32; 12] = [0; 12];

    for bit_string in bit_strings {
        for (i, bit) in bit_string.char_indices() {
            match bit {
                '0' => zero_digit_counts[i] += 1,
                '1' =>  one_digit_counts[i] += 1,
                _ => panic!("Unknown bit")
            }
        }
    }

    // Made of the most common bits
    let mut gamma_rate: u32 = 0;
    // Made of the least common bits
    let mut epsilon: u32 = 0;
    for i in 0..12 {
        if zero_digit_counts[i] > one_digit_counts[i] {
            gamma_rate <<= 1;
            epsilon = (epsilon << 1) | 1;
        } else {
            epsilon <<= 1;
            gamma_rate = (gamma_rate << 1) | 1;
        }
    }
    println!("{}", gamma_rate * epsilon);
    Ok(())
}
