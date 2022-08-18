use core::hash::Hash;
use std::collections::{HashMap, HashSet};

pub struct Graph<Nd: Eq + Hash + Clone> {
    nodes: HashSet<Nd>,
    edges: HashMap<Nd, HashSet<Nd>>,
}

impl<Nd: Eq + Hash + Clone> Graph<Nd> {
    pub fn new() -> Graph<Nd> {
        Graph {
            nodes: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, source: Nd, end: Nd, bi: bool) {
        self.nodes.insert(source.clone());
        self.nodes.insert(end.clone());

        let edges = self
            .edges
            .entry(source.clone())
            .or_insert(HashSet::<Nd>::new());
        edges.insert(end.clone());

        if bi {
            let edges = self.edges.entry(end).or_insert(HashSet::<Nd>::new());
            edges.insert(source);
        }
    }

    pub fn get_edges(&self, node: Nd) -> HashSet<Nd> {
        self.edges.get(&node).unwrap_or(&HashSet::new()).clone()
    }
}

pub trait DFS<Nd> {
    fn dfs(&self, cur: Nd, end: Nd, path: Vec<Nd>) -> Vec<Vec<Nd>>;
}

use String as Nd;

impl DFS<Nd> for Graph<Nd> {
    fn dfs(&self, cur: Nd, end: Nd, path: Vec<Nd>) -> Vec<Vec<Nd>> {
        let mut paths = Vec::new();

        if cur == "start" && path.len() > 1 {
            return paths;
        } else if cur.to_lowercase() == cur && path.contains(&cur) {
            // if 2 of any small cave already present
            let mut counts: HashMap<_, u32> = HashMap::new();

            for node in path.iter() {
                if &node.to_lowercase() == node {
                    *counts.entry(node).or_default() += 1;
                }
            }

            if counts.values().max().unwrap() >= &2u32 {
                return paths;
            }
        }

        let mut path = path.clone();
        path.push(cur.clone());

        if cur == end {
            paths.push(path);
        } else {
            let children = self.get_edges(cur);

            for child in children.into_iter() {
                paths.extend(self.dfs(child.clone(), end.clone(), path.clone()));
            }
        }

        paths
    }
}
