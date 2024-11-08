use advent_of_code::utils::plane::{Line, Point};
use itertools::Itertools;
use num::{range_step, range_step_inclusive, Integer};
use rayon::prelude::*;
use std::{
    cmp::{max, min},
    collections::HashMap,
    fmt::{Debug, Display},
};
advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<impl Debug + Display + num::Integer> {
    let points = input
        .lines()
        .map(|l| l.split_whitespace().collect_tuple::<(_, _, _)>().unwrap())
        .map(|parts| {
            let start = parts.0.split(",").collect_tuple::<(_, _)>().unwrap();
            let end = parts.2.split(",").collect_tuple::<(_, _)>().unwrap();
            let start = Point::from_xy(start.0.parse().unwrap(), start.1.parse().unwrap());
            let end = Point::from_xy(end.0.parse().unwrap(), end.1.parse().unwrap());
            Line::from(start, end)
        })
        .filter(|l| l.is_horizontal() || l.is_vertical())
        .tuple_combinations::<(_, _)>()
        .map(|(l1, l2)| match l1.overlap(&l2) {
            Some(overlap) => overlap.to_points(),
            None => match l1.intersection(&l2) {
                Some(p) => vec![p],
                None => vec![],
            },
        })
        .flatten()
        .collect_vec();
    let mut res: HashMap<&Point, u32> = HashMap::new();
    for point in &points {
        res.insert(&point, res.get(&point).or(Some(&0)).unwrap() + 1);
    }
    Some(res.len())
}

pub fn part_two(input: &str) -> Option<impl Debug + Display + num::Integer> {
    let points = input
        .lines()
        .map(|l| l.split_whitespace().collect_tuple::<(_, _, _)>().unwrap())
        .map(|parts| {
            let start = parts.0.split(",").collect_tuple::<(_, _)>().unwrap();
            let end = parts.2.split(",").collect_tuple::<(_, _)>().unwrap();
            let start = Point::from_xy(start.0.parse().unwrap(), start.1.parse().unwrap());
            let end = Point::from_xy(end.0.parse().unwrap(), end.1.parse().unwrap());
            Line::from(start, end)
        })
        .tuple_combinations::<(_, _)>()
        .map(|(l1, l2)| match l1.overlap(&l2) {
            Some(overlap) => overlap.to_points(),
            None => match l1.intersection(&l2) {
                Some(p) => vec![p],
                None => vec![],
            },
        })
        .flatten()
        .collect_vec();
    let mut res: HashMap<&Point, u32> = HashMap::new();
    for point in &points {
        if !point.x.is_integer() || !point.y.is_integer() {
            continue;
        }
        res.insert(&point, res.get(&point).or(Some(&0)).unwrap() + 1);
    }
    Some(res.len())
}

#[cfg(test)]
mod tests {
    use advent_of_code::utils::plane::{Line, Point};

    use super::*;

    #[test]
    fn test_part_one() {
        // let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
        let l1 = Line::from(Point::from_xy(0, 0), Point::from_xy(2, 2));
        let l2 = Line::from(Point::from_xy(2, 0), Point::from_xy(0, 2));
        let p = l1.intersection(&l2);
        println!("{:?}", p);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
