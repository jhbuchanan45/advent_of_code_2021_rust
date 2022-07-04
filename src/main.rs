use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // using y, x indexing
    let mut heightmap: Vec<Vec<u8>> = Vec::new();

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            let cur = line
                .unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>();

            heightmap.push(cur);
        }
    }

    // find and score low points
    let y_max = heightmap.len() - 1;
    let x_max = heightmap[0].len() - 1;
    let mut total_score = 0;

    for (y, row) in heightmap.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            let mut lowest = true;

            // top if not in row 0
            if y != 0 {
                if heightmap[y - 1][x] <= *height {
                    lowest = false;
                }
            }

            // left if not on column 0
            if x != 0 {
                if row[x - 1] <= *height {
                    lowest = false;
                }
            }

            // right if not in column max
            if x != x_max {
                if row[x + 1] <= *height {
                    lowest = false;
                }
            }

            // down if not bottom row
            if y != y_max {
                if heightmap[y + 1][x] <= *height {
                    lowest = false;
                }
            }

            if lowest {
                total_score += 1 + *height as u32;
            }
        }
    }

    println!("{}", total_score);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
