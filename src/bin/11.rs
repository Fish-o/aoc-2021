use advent_of_code::utils::matrix::{Matrix, Metric};
use itertools::Itertools;
use rayon::prelude::*;
use std::{
    fmt::{Debug, Display},
    mem,
};
advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let mut m = Matrix::from_str(input, "\n", "").parse::<usize>().unwrap();
    let mut flashes = 0;
    for _ in 0..100 {
        let mut increment = m.positions();
        while !increment.is_empty() {
            let mut new_increment = vec![];
            for p in increment {
                let c = m.get_pos_mut(&p).unwrap();
                *c += 1;
                if *c == 10 {
                    flashes += 1;
                    new_increment.append(&mut m.neighbours_in_range(&p, &Metric::Chebyshev, 1));
                }
            }
            increment = new_increment;
        }
        m.cells_mut()
            .into_iter()
            .filter(|c| **c >= 10 as usize)
            .for_each(|c| *c = 0);
    }
    Some(flashes)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut m = Matrix::from_str(input, "\n", "").parse::<usize>().unwrap();
    for i in 1.. {
        let mut increment = m.positions();
        let mut flashes = 0;
        while !increment.is_empty() {
            let mut new_increment = vec![];
            for p in increment {
                let c = m.get_pos_mut(&p).unwrap();
                *c += 1;
                if *c == 10 {
                    flashes += 1;
                    if flashes == m.count() {
                        return Some(i);
                    }
                    new_increment.append(&mut m.neighbours_in_range(&p, &Metric::Chebyshev, 1));
                }
            }
            increment = new_increment;
        }
        m.cells_mut()
            .into_iter()
            .filter(|c| **c >= 10 as usize)
            .for_each(|c| *c = 0);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1656));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(195));
    }
}
