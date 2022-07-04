use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

use std::path::Path;

fn main() {
    let mut displays = Vec::new();

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            let cur = line.unwrap();
            let cur = cur.split_once("|").unwrap();
            let cur = (cur.0.trim(), cur.1.trim());

            let sample = cur
                .0
                .split(" ")
                .map(|digit| digit.chars().collect::<HashSet<char>>())
                .collect::<Vec<_>>();

            let result = cur
                .1
                .split(" ")
                .map(|digit| digit.chars().collect::<HashSet<char>>())
                .collect::<Vec<_>>();

            displays.push((sample, result));
        }
    }

    let displays = displays;
    let mut decoders = Vec::new();

    for display in &displays {
        let mut decoder: HashMap<u8, _> = HashMap::new();

        // grab known samples for comparisons
        for sample in display.0.clone() {
            match sample.len() {
                2 => drop(decoder.insert(1, sample)),
                3 => drop(decoder.insert(7, sample)),
                4 => drop(decoder.insert(4, sample)),
                7 => drop(decoder.insert(8, sample)),
                5 | 6 => (),
                _ => println!("Error: Invalid number length for 7 segment display!"),
            }
        }

        // handle ambiguous cases
        for sample in display.0.clone() {
            match sample.len() {
                5 => {
                    if decoder.get(&1).unwrap().intersection(&sample).count() == 2 {
                        // match 3
                        decoder.insert(3, sample);
                    } else {
                        match decoder.get(&4).unwrap().intersection(&sample).count() {
                            2 => drop(decoder.insert(2, sample)),
                            3 => drop(decoder.insert(5, sample)),
                            _ => (),
                        }
                    }
                }

                6 => {
                    if decoder.get(&1).unwrap().intersection(&sample).count() == 1 {
                        // match 6
                        decoder.insert(6, sample);
                    } else {
                        match decoder.get(&4).unwrap().intersection(&sample).count() {
                            3 => drop(decoder.insert(0, sample)),
                            4 => drop(decoder.insert(9, sample)),
                            _ => (),
                        }
                    }
                }
                2 | 3 | 4 | 7 => (),
                _ => println!("Error: Invalid number length for 7 segment display!"),
            }
        }

        decoders.push(decoder);
    }

    let mut total = 0;

    for (display, decoder) in displays.into_iter().zip(decoders) {
        let result = display
            .1
            .into_iter()
            .map(|digit| match_number(digit, &decoder).unwrap() as u32).map(|d| char::from_digit(d, 10).unwrap())
            .collect::<String>();

        total += result.parse::<u32>().unwrap();
    }

    println!("{}", total);
}

fn match_number(digit: HashSet<char>, decoder: &HashMap<u8, HashSet<char>>) -> Result<u8, String> {
    for (number, set) in decoder.clone().drain() {
        if digit == set {
            return Ok(number);
        }
    }

    Err(String::from("No match found!"))
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
