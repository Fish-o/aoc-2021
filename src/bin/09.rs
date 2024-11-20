use advent_of_code::utils::matrix::{Matrix, Pos};
use itertools::Itertools;
use rayon::prelude::*;
use std::fmt::{Debug, Display};
advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<impl Debug + Display + num::Integer> {
    let m = Matrix::from_str(input, "\n", "").parse::<usize>().unwrap();
    Some(
        m.enumerate()
            .into_iter()
            .filter(|(p, v)| m.neighbours(p).iter().all(|n| n > v))
            .map(|(p, h)| h + 1)
            .sum::<usize>(),
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
