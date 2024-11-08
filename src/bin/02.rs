
use std::fmt::{Debug, Display};

use advent_of_code::utils::parsing;
use itertools::Itertools;
use rayon::prelude::*;
advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<impl Debug + Display + num::Integer> { 
    let mut x: isize = 0;
    let mut y: isize = 0;
    for line in input.lines() {
        let s = line.split_once(" ").unwrap();
        let d = s.1.parse::<isize>().unwrap();
        match s.0 {
            "forward" => {x+= d}
            "up" => {y-= d}
            "down" => {y+=d}
            _=>{}
        }
    }
    return Some(x * y);
}

pub fn part_two(input: &str) -> Option<impl Debug + Display + num::Integer> {
    let mut aim: isize = 0;
    let mut x: isize = 0;
    let mut y: isize = 0;
    for line in input.lines() {
        let s = line.split_once(" ").unwrap();
        let d = s.1.parse::<isize>().unwrap();
        match s.0 {
            "forward" => {x+= d;y+=aim*d;}
            "up" => {aim-= d}
            "down" => {aim+=d}
            _=>{}
        }
    }
    return Some(x * y); 
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
