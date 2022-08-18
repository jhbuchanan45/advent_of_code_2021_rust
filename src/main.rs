mod graph;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::graph::{Graph, DFS};

fn main() {
    let mut graph: Graph<String> = Graph::new();

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            let cur = line.unwrap();
            let cur = cur.split_once('-').unwrap();

            graph.add_edge(cur.0.to_owned(), cur.1.to_owned(), true);
        }
    }

    let paths = graph.dfs("start".to_string(), "end".to_string(), Vec::new());

    println!("{:?}", paths.iter().count());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
