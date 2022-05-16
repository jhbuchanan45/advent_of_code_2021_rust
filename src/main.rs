use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut depth = 0;
    let mut horizontal = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            let line = line.unwrap();
            let line_val = line.split_whitespace().take(2).collect::<Vec<&str>>();
            
            if let [cmd, amt] = line_val[..] {
                let amt = amt.parse::<i32>().unwrap();

                match cmd {
                    "forward" => horizontal += amt,
                    "up" => depth -= amt,
                    "down" => depth += amt,
                    _ => panic!("Unrecognised command!")
                }
            }
        }
    }

    println!("Final Depth: {}", depth);
    println!("Final Horizontal Position: {}", horizontal);
    println!("Result: {}", depth * horizontal);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
