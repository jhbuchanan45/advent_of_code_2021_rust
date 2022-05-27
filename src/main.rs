use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Map;
use std::path::Path;

fn main() {
    let days = 256;
    // let mut generation_counts = HashMap::new();
    let mut lanternfish = Vec::new();

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            let clocks = line.unwrap();

            for clock in clocks.split(',') {
                lanternfish.push(clock.parse::<u8>().unwrap());
            }
        }
    }

    for _ in 0..days {
        for (i, &clock) in lanternfish.clone().iter().enumerate() {
            if clock == 0 {
                lanternfish.push(8);
                lanternfish[i] = 6;
            } else {
                lanternfish[i] -= 1
            }
        }
    }

    println!("{:?}", lanternfish.len());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
