use itertools::Itertools;
use num::Integer;
use rayon::prelude::*;
use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign},
};
advent_of_code::solution!(6);

const MAX_DAYS: usize = 7;

pub fn part_one(input: &str) -> Option<usize> {
    let mut new_fishes_per_day = [0; MAX_DAYS];
    let mut baby_queue = vec![0, 0];
    for delay in input.trim().split(",").map(|c| c.parse::<usize>().unwrap()) {
        new_fishes_per_day.get_mut(delay).unwrap().inc();
    }
    println!("{:?}", new_fishes_per_day);
    for day in 0..256 {
        let i = day % MAX_DAYS;
        let young_uns = new_fishes_per_day.get(i).unwrap().clone();
        baby_queue.push(young_uns);

        let to_add = baby_queue.remove(0);
        new_fishes_per_day.get_mut(i).unwrap().add_assign(to_add);
    }

    Some(new_fishes_per_day.iter().sum::<usize>() + baby_queue.iter().sum::<usize>())
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
        assert_eq!(result, Some(5934));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
