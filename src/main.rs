use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let days = 256;
    let mut timer_counts: [u64; 9] = [0; 9];

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            let clocks = line.unwrap();

            for clock in clocks.split(',') {
                timer_counts[clock.parse::<usize>().unwrap()] += 1;
            }
        }
    }

    for _ in 0..days {
        let mut new_timers = [0; 9];
        for (i, count) in timer_counts.iter().enumerate() {
            if i == 0 {
                new_timers[6] += count;
                new_timers[8] += count;
            } else {
                new_timers[i-1] += count;
            }
        }

        timer_counts = new_timers;
    }

    let total = timer_counts.iter().sum::<u64>(); 

    println!("{}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
