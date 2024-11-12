use itertools::Itertools;
use num::{FromPrimitive, Rational32};
use rayon::prelude::*;
use std::fmt::{Debug, Display};
advent_of_code::solution!(7);

fn get_cost(crabs: &Vec<usize>, pos: usize) -> usize {
    crabs.iter().map(|v| v.abs_diff(pos)).sum()
}
fn get_cost2(crabs: &Vec<usize>, pos: usize) -> u32 {
    crabs
        .iter()
        .map(|v| v.abs_diff(pos))
        .map(|c| Rational32::from_usize(c).unwrap())
        .map(|c| ((c + 1) * c) / 2)
        .map(|c| c.to_integer() as u32)
        .sum::<u32>()
}

pub fn part_one(input: &str) -> Option<usize> {
    let crabs = input
        .trim()
        .split(",")
        .map(|c| c.parse::<usize>().unwrap())
        .collect_vec();
    let mut min = 0;
    let mut max = 1000;
    loop {
        let mid = (min + max) / 2;
        let a = get_cost(&crabs, mid - 1);
        let b = get_cost(&crabs, mid);
        let c = get_cost(&crabs, mid + 1);
        if a > b && c > b {
            return Some(b);
        } else if a > b {
            min = mid;
        } else if c > b {
            max = mid;
        }
    }
}

pub fn part_two(input: &str) -> Option<impl Debug + Display + num::Integer> {
    let crabs = input
        .trim()
        .split(",")
        .map(|c| c.parse::<usize>().unwrap())
        .collect_vec();
    let mut min = 0;
    let mut max = 1000;
    loop {
        let mid = (min + max) / 2;
        let a = get_cost2(&crabs, mid - 1);
        let b = get_cost2(&crabs, mid);
        let c = get_cost2(&crabs, mid + 1);
        if a > b && c > b {
            return Some(b);
        } else if a > b {
            min = mid;
        } else if c > b {
            max = mid;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
