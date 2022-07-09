mod cavern;
mod pos;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::cavern::{Cavern, Charge};

fn main() {
    let mut cavern = Cavern::<usize, u8>::default();

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            let cur = line.unwrap().chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect::<Vec<u8>>();
            
            cavern.charges.push(cur);
        }
    }

    for _ in 0..100 {
        cavern.charge_all();
        cavern.process_queue();
    }

    println!("{}", cavern.flash_count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
