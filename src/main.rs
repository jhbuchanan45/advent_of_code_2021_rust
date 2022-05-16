use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut prev_sum = i32::MAX;
    let mut window = [0, 0, 0];
    let mut increase_count = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for (i, line) in lines.enumerate() {
            let new_val = line.unwrap().parse::<i32>().unwrap();
            shift_window(&mut window, new_val);

            // only after window is full, i points to right side of window
            if i > 1 {
                let cur_sum = window.iter().sum();

                if prev_sum < cur_sum {
                    increase_count += 1;
                }

                prev_sum = cur_sum;
            }
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

fn shift_window<V, const N: usize>(window: &mut [V; N], new_val: V)
where
    V: Copy,
{
    for i in 1..window.len() {
        window[i - 1] = window[i];
    }

    window[window.len() - 1] = new_val;
}
