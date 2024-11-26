use itertools::Itertools;
use petgraph::{
    algo::all_simple_paths,
    graph::{NodeIndex, UnGraph},
};
use rayon::prelude::*;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    ops::Index,
};
advent_of_code::solution!(12);
pub fn permute(
    path: &Vec<usize>,
    g: &UnGraph<u32, ()>,
    nodes: &HashMap<usize, String>,
    part_two: bool,
) -> Vec<Vec<usize>> {
    let pos = path.iter().last().unwrap();
    let neighbours = g.neighbors(NodeIndex::new(*pos));
    let mut paths = vec![];
    for neighbour in neighbours {
        let name = nodes.get(&neighbour.index()).unwrap();
        let mut new_path = path.clone();
        new_path.push(neighbour.index());
        if name == &"end".to_owned() {
            paths.push(new_path);
            continue;
        }
        if name == &"start".to_owned() {
            continue;
        }

        if !part_two {
            if &name.to_uppercase() != name && path.contains(&neighbour.index()) {
                continue;
            }
        } else if &name.to_uppercase() != name && path.contains(&neighbour.index()) {
            if path
                .iter()
                .map(|i| nodes.get(i).unwrap())
                .sorted()
                .tuple_windows()
                .any(|(a, b)| a == b && &a.to_lowercase() == a)
            {
                continue;
            };
        }
        let mut new_paths = permute(&new_path, g, nodes, part_two);
        paths.append(&mut new_paths);
    }
    paths
}
pub fn part_one(input: &str) -> Option<usize> {
    let nodes = input
        .trim()
        .lines()
        .flat_map(|l| l.trim().split('-').collect_vec())
        .sorted()
        .dedup()
        .enumerate();

    let nodes_rev = nodes
        .clone()
        .map(|(k, v)| (k, v.to_owned()))
        .collect::<HashMap<_, _>>();
    let nodes = nodes
        .map(|(k, v)| (v.to_owned(), k))
        .collect::<HashMap<_, _>>();
    let g = UnGraph::<u32, ()>::from_edges(
        input
            .lines()
            .map(|l| l.split_once("-").unwrap())
            .map(|(a, b)| (*nodes.get(a).unwrap() as u32, *nodes.get(b).unwrap() as u32)),
    );

    let r = permute(
        &vec![nodes.get("start").unwrap().clone()],
        &g,
        &nodes_rev,
        false,
    );
    Some(r.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let nodes = input
        .trim()
        .lines()
        .flat_map(|l| l.trim().split('-').collect_vec())
        .sorted()
        .dedup()
        .enumerate();

    let nodes_rev = nodes
        .clone()
        .map(|(k, v)| (k, v.to_owned()))
        .collect::<HashMap<_, _>>();
    let nodes = nodes
        .map(|(k, v)| (v.to_owned(), k))
        .collect::<HashMap<_, _>>();
    let g = UnGraph::<u32, ()>::from_edges(
        input
            .lines()
            .map(|l| l.split_once("-").unwrap())
            .map(|(a, b)| (*nodes.get(a).unwrap() as u32, *nodes.get(b).unwrap() as u32)),
    );

    let r = permute(
        &vec![nodes.get("start").unwrap().clone()],
        &g,
        &nodes_rev,
        true,
    );
    Some(r.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3509));
    }
}
