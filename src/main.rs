use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut crabs = Vec::new();
    let mut max_len = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            let line = line.unwrap();

            for crab_pos in line.split(',') {
                let pos = crab_pos.parse::<usize>().unwrap();
                crabs.push(pos);

                if max_len < pos {
                    max_len = pos + 1;
                }
            }
        }
    }

    let mut crab_counts = vec![0; max_len];

    for crab in crabs {
        crab_counts[crab] += 1;
    }

    // get score for 0 pos
    let mut left_cost = 0;
    let mut right_cost = 0;
    let mut right_count = 0;
    let mut left_count = 0;

    for i in 1..max_len {
        right_cost += i * crab_counts[i];
        right_count += crab_counts[i];
    }

    let mut min_cost = right_cost;

    // move right, updating score for each left and right sides
    for i in 1..max_len {
        right_cost -= right_count;
        right_count -= crab_counts[i];

        left_count += crab_counts[i - 1];
        left_cost += left_count;

        if left_cost + right_cost < min_cost {
            min_cost = left_cost + right_cost;
        }
    }

    println!("{:?}", min_cost);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
