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
    let mut min_cost = i32::MAX;

    for crab in crabs {
        crab_counts[crab] += 1;
    }

    // calculate score for each pos
    for pos in 0..max_len {
        let mut score = 0;

        // get score
        for i in 0..max_len {
            let dist = (pos as i32 - i as i32).abs();
            score += crab_counts[i] * ((dist * (dist + 1)) / 2);
        }

        if score < min_cost {
            min_cost = score;
        };
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
