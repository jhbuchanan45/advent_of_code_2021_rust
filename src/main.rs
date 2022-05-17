use core::panic;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut gamma = 0;
    let mut epsilon = 0;
    let mut count_ones = [0; 12];
    let mut total = 0;

    // count ones
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            let line = line.unwrap();

            for (i, bit) in line.chars().enumerate() {
                match bit {
                    '1' => count_ones[i] += 1,
                    '0' => continue,
                    _ => panic!("Character was not '0' or '1'!"),
                }
            }

            total += 1;
        }
    }

    // get most common for each column
    for one_count in count_ones {
        gamma = gamma << 1;
        epsilon = epsilon << 1;

        if one_count > total / 2 {
            gamma += 1
        } else {
            epsilon += 1
        }
    }

    println!("Final Gamma: {}", gamma);
    println!("Final Epsilon: {}", epsilon);
    println!("Power Consumption: {}", gamma * epsilon);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
