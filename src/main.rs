use std::any::Any;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut displays: Vec<(Vec<String>, Vec<String>)> = Vec::new();
    

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            let cur = line.unwrap();
            let cur = cur.split_once("|").unwrap();

            let sample = cur.0.split(" ").map(String::from).collect::<Vec<String>>();
            let result = cur.1.split(" ").map(String::from).collect::<Vec<String>>();

            displays.push((sample, result));
        }
    }

    // find number of 1's, 4's, 7's, and 8's
    let mut unique_count = 0;
    
    for display in displays {
        for result_digit in display.1 {
            match result_digit.len() {
                2 | 4 | 3 | 7 => unique_count += 1,
                _ => (),
            }
        }
    }

    println!("{}", unique_count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
