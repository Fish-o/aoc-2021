use itertools::Itertools;
use petgraph::{
    algo::all_simple_paths,
    graph::{NodeIndex, UnGraph},
};
use rayon::prelude::*;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};
advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    let nodes = input
        .trim()
        .lines()
        .flat_map(|l| l.trim().split('-').collect_vec())
        .sorted()
        .dedup()
        .enumerate()
        .map(|(k, v)| (v.to_owned(), k))
        .collect::<HashMap<_, _>>();
    let g = UnGraph::<u32, ()>::from_edges(
        input
            .lines()
            .map(|l| l.split_once("-").unwrap())
            .map(|(a, b)| (*nodes.get(a).unwrap() as u32, *nodes.get(b).unwrap() as u32)),
    );
    Some(
        all_simple_paths::<Vec<_>, _>(
            &g,
            NodeIndex::new(*nodes.get("start").unwrap()),
            NodeIndex::new(*nodes.get("end").unwrap()),
            0,
            None,
        )
        .count(),
    )
}

pub fn part_two(input: &str) -> Option<impl Debug + Display + num::Integer> {
    None::<u64>
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
        assert_eq!(result, None);
    }
}
