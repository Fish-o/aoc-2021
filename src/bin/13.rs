use advent_of_code::utils::matrix::{Matrix, Pos, Shape};
use itertools::Itertools;
use rayon::prelude::*;
use std::fmt::{Debug, Display};
advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<usize> {
    let (points, inst) = input.trim().split_once("\n\n").unwrap();

    let mut m = Matrix::from_points(
        points
            .lines()
            .map(|l| l.split_once(",").unwrap())
            .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
            .map(|(a, b)| Pos::from_xy(a, b))
            .map(|p| (p, ()))
            .collect_vec(),
    );
    for (d, c) in inst
        .lines()
        .map(|l| l.split("along ").last().unwrap().split_once("=").unwrap())
        .take(1)
    {
        let c = c.parse::<usize>().unwrap();
        m = match d {
            "y" => {
                let a = m.slice(&Shape::Above(c));
                let b = m.slice(&Shape::Below(c));
                let b = b.flip_hor();
                a.merge(
                    &b,
                    |_, a, b| match (a, b) {
                        (None, None) => None,
                        _ => Some(()),
                    },
                    |_| panic!(),
                )
            }
            "x" => {
                let a = m.slice(&Shape::Left(c));
                let b = m.slice(&Shape::Right(c));
                let b = b.flip_ver();
                a.merge(
                    &b,
                    |_, a, b| match (a, b) {
                        (None, None) => None,
                        _ => Some(()),
                    },
                    |_| panic!(),
                )
            }
            _ => unreachable!(),
        };
    }
    Some(
        m.enumerate()
            .iter()
            .fold(0, |r, c| if c.1.is_some() { r + 1 } else { r }),
    )
}

pub fn part_two(input: &str) -> Option<impl Debug + Display + num::Integer> {
    let (points, inst) = input.trim().split_once("\n\n").unwrap();

    let mut m = Matrix::from_points(
        points
            .lines()
            .map(|l| l.split_once(",").unwrap())
            .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
            .map(|(a, b)| Pos::from_xy(a, b))
            .map(|p| (p, ()))
            .collect_vec(),
    );
    for (d, c) in inst
        .trim()
        .lines()
        .map(|l| l.split("along ").last().unwrap().split_once("=").unwrap())
    {
        let c = c.parse::<usize>().unwrap();
        m = match d {
            "y" => {
                let a = m.slice(&Shape::Above(c));
                let b = m.slice(&Shape::Below(c));
                let mut b = b.flip_hor();
                while a.height() > b.height() {
                    b.insert_row(0, vec![None; b.width()]);
                }
                while a.height() < b.height() {
                    b.remove_row(0);
                }

                a.merge(
                    &b,
                    |_, a, b| match (a, b) {
                        (None, None) => None,
                        _ => Some(()),
                    },
                    |_| panic!(),
                )
            }
            "x" => {
                let a = m.slice(&Shape::Left(c));
                let b = m.slice(&Shape::Right(c));
                let mut b = b.flip_ver();
                while a.width() > b.width() {
                    b.insert_col(0, vec![None; b.height()]);
                }
                while a.width() < b.width() {
                    b.remove_col(0);
                }
                a.merge(
                    &b,
                    |_, a, b| match (a, b) {
                        (None, None) => None,
                        _ => Some(()),
                    },
                    |_| panic!(),
                )
            }
            _ => unreachable!(),
        };
    }
    let p = m.map(|_, c| match c {
        Some(_) => "#",
        None => ".",
    });
    println!("{p:}");
    println!("\n");
    None::<usize>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(17));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
