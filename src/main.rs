mod draw;

use std::fs::File;
use std::io::{self, BufRead};
use std::ops::{Range, RangeInclusive};
use std::path::Path;
use crate::draw::draw_line;


fn main() {
    let mut vents = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            let line = line
                .unwrap()
                .split(" -> ")
                .map(|pnt| point_str_to_tuple(pnt))
                .collect::<Vec<_>>();

            if line.len() == 2 {
                vents.push((line[0], line[1]));

                if line[0].0 > max_x {
                    max_x = line[0].0;
                }

                if line[1].0 > max_x {
                    max_x = line[1].0;
                }

                if line[0].1 > max_y {
                    max_y = line[0].1;
                }

                if line[1].1 > max_y {
                    max_y = line[1].1;
                }
            } else {
                panic!("too many points")
            }
        }
    }

    let mut grid = vec![vec![0; (max_y + 1) as usize]; (max_x + 1) as usize];

    for vent in &vents {
        draw_line(&mut grid, vent.0, vent.1);
    }

    let total_dangerous = grid.iter().flatten().filter(|&&point| point > 1).count();

    println!("Total Dangerous: {}", total_dangerous);
}

fn point_str_to_tuple(point: &str) -> (u32, u32) {
    let point = point.split(',').collect::<Vec<_>>();

    if point.len() == 2 {
        return (
            point[0].parse::<u32>().unwrap(),
            point[1].parse::<u32>().unwrap(),
        );
    } else {
        panic!("x1,y1 pair must be pair")
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
