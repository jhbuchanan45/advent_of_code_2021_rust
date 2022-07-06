use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Node {
    y: usize,
    x: usize,
}

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

    // transform grid into true/false map
    let mut grid = heightmap
        .iter()
        .map(|row| row.iter().map(|h| h == &9).collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();

    // count each basin
    let mut basin_sizes = Vec::new();
    let mut last_basin = Node { y: 0, x: 0 };

    for y in (last_basin.y)..grid.len() {
        for x in 0..grid[0].len() {
            if !grid[y][x] {
                // find and store size of next basin
                last_basin = Node { y: y, x: x };
                basin_sizes.push(dfs(&mut grid, &last_basin))
            }
        }
    }

    basin_sizes.sort();
    basin_sizes.reverse();

    println!("{}", basin_sizes[0] * basin_sizes[1] * basin_sizes[2]);
}

fn dfs(grid: &mut Vec<Vec<bool>>, node: &Node) -> u32 {
    let mut count = 0;

    grid[node.y][node.x] = true;
    count += 1;

    // top if not in row 0
    if node.y != 0 {
        if !grid[node.y - 1][node.x] {
            count += dfs(
                grid,
                &Node {
                    y: node.y - 1,
                    x: node.x,
                },
            )
        }
    }

    // left if not on column 0
    if node.x != 0 {
        if !grid[node.y][node.x - 1] {
            count += dfs(
                grid,
                &Node {
                    y: node.y,
                    x: node.x - 1,
                },
            )
        }
    }

    // right if not in column max
    if node.x != grid[0].len() - 1 {
        if !grid[node.y][node.x + 1] {
            count += dfs(
                grid,
                &Node {
                    y: node.y,
                    x: node.x + 1,
                },
            )
        }
    }

    // down if not bottom row
    if node.y != grid.len() - 1 {
        if !grid[node.y + 1][node.x] {
            count += dfs(
                grid,
                &Node {
                    y: node.y + 1,
                    x: node.x,
                },
            )
        }
    }

    return count;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
