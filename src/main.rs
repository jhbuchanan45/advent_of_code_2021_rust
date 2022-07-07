use core::str::Chars;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const BRACKETS: ([char; 4], [char; 4]) = (['(', '[', '{', '<'], [')', ']', '}', '>']);

struct BracketResult {
    corrupt_char: Option<char>,
    unclosed_brackets: Vec<usize>,
}

fn main() {
    let mut unclosed_brackets = Vec::new();

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            let line = line.unwrap();

            let mut bracket_result = corruption_check(&line.chars());

            if let Some(_corrupt_ch) = bracket_result.corrupt_char {
                continue;
            } else {
                bracket_result.unclosed_brackets.reverse();
                unclosed_brackets.push(bracket_result.unclosed_brackets);
            }
        }
    }

    let mut score = unclosed_brackets
        .iter()
        .map(|v| v.iter().fold(0, |acc, &ch| (acc * 5) + ch + 1))
        .collect::<Vec<usize>>();

    score.sort();

    println!("{}", score.get(score.len() / 2).unwrap());
}

fn corruption_check(line: &Chars) -> BracketResult {
    let mut open_stack = Vec::new();

    for ch in line.clone() {
        if BRACKETS.0.contains(&ch) {
            open_stack.push(BRACKETS.0.iter().position(|v| v == &ch).unwrap());
        } else {
            let expected_index = open_stack.pop().unwrap();

            if expected_index == BRACKETS.1.iter().position(|v| v == &ch).unwrap() {
                continue;
            } else {
                return BracketResult {
                    corrupt_char: Option::Some(ch),
                    unclosed_brackets: open_stack,
                };
            }
        }
    }

    return BracketResult {
        corrupt_char: Option::None,
        unclosed_brackets: open_stack,
    };
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
