use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use core::str::Chars;

fn main() {
    let mut corrupt_closers = Vec::new();
    let mut clean_lines = Vec::new();

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            let line = line.unwrap();

            if let Some(corrupt_ch) = corruption_check(&line.chars()) {
                corrupt_closers.push(corrupt_ch);
            } else {
                clean_lines.push(line);
                continue;
            }
        }
    }

    let score: u32 = corrupt_closers.iter().map(|ch| match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0
    }).sum();

    println!("{}", score);
}

fn corruption_check(line: &Chars) -> Option<char> {
    let brackets = (['(', '{', '[', '<'], [')', '}', ']', '>']);
    let mut open_stack = Vec::new();

    for ch in line.clone() {
        if brackets.0.contains(&ch) {
            open_stack.push(brackets.0.iter().position(|v| v == &ch).unwrap());
        } else {
            let expected_index = open_stack.pop().unwrap();

            if expected_index == brackets.1.iter().position(|v| v == &ch).unwrap() {
                continue;
            } else {
                return Option::Some(ch);
            }
        }
    }

    return Option::None;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
