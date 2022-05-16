use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut prev = std::i32::MAX;
    let mut increase_count = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            let cur = line.unwrap().parse::<i32>().unwrap();
            if prev < cur {
                increase_count += 1;
            }
            prev = cur;
        }
    }

    println!("{}", increase_count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
