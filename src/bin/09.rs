use advent_of_code::utils::matrix::{Matrix, Pos};
use itertools::Itertools;
use rayon::prelude::*;
use std::fmt::{Debug, Display};
advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let m = Matrix::from_str(input, "\n", "").parse::<usize>().unwrap();
    Some(
        m.enumerate()
            .into_iter()
            .filter(|(p, v)| m.neighbours(p).iter().all(|n| n > v))
            .map(|(p, h)| h + 1)
            .sum::<usize>(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let m = Matrix::from_str(input, "\n", "").parse::<usize>().unwrap();
    let seeds = m
        .enumerate()
        .iter()
        .filter(|(p, v)| v < &&(9 as usize))
        .map(|(p, _)| p.clone())
        .collect_vec();
    let mut res = m.flood_regions(
        &seeds,
        |m, p, r| {
            m.neighbour_positions(p)
                .into_iter()
                .filter(|p| m.get_pos(&p).unwrap() < &(9 as usize))
                .collect_vec()
        },
        true,
        true,
    );
    Some(res.iter().map(|r| r.len()).sorted().rev().take(3).product())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1134));
    }
}
