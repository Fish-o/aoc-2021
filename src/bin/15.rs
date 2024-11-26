use rayon::prelude::*;
use std::fmt::{Debug, Display};
use itertools::Itertools;
advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<impl Debug + Display + num::Integer> {
    None::<u64>
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