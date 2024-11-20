use advent_of_code::template::commands::solve;
use itertools::Itertools;
use rayon::prelude::*;
use std::fmt::{Debug, Display};
advent_of_code::solution!(8);

pub fn translate(s: &str, k: &Vec<&char>) -> String {
    let mut res = vec![];
    for char in s.chars() {
        res.push(
            k.iter()
                .nth(char as usize - 'A' as usize)
                .unwrap()
                .to_string(),
        );
    }
    res.sort();
    res.into_iter().join("")
}
pub fn solve_entry(s: &str) -> usize {
    let l = ('A'..='G').collect_vec();
    let options = l.iter().permutations(7).collect_vec();
    let letters = vec![
        "ABCEFG", "CF", "ACDEG", "ACDFG", "BCDF", "ABDFG", "ABDEFG", "ACF", "ABCDEFG", "ABCDFG",
    ];
    let mut i = s.split("|");
    let inp = i.next().unwrap().trim();
    let out = i.next().unwrap().trim();
    let parts = vec![
        inp.split_whitespace().collect_vec(),
        out.split_whitespace().collect_vec(),
    ]
    .concat();
    let option = options
        .into_iter()
        .find(|o| {
            for part in &parts {
                let translated = translate(&part, &o);
                if !letters.contains(&translated.as_ref()) {
                    return false;
                }
            }
            return true;
        })
        .unwrap();
    out.split_whitespace()
        .map(|c| translate(c, &option))
        .map(|t| letters.iter().position(|v| v == &t).unwrap())
        .join("")
        .parse::<usize>()
        .unwrap()
}
pub fn part_one(input: &str) -> Option<impl Debug + Display + num::Integer> {
    Some(
        input
            .trim()
            .lines()
            .map(|l| l.split("|").skip(1).next().unwrap())
            .map(|v| v.split_whitespace().collect_vec())
            .map(|v| {
                v.iter()
                    .filter(|v| vec![2, 3, 4, 7].contains(&v.len()))
                    .count()
            })
            .sum::<usize>(),
    )
}

pub fn part_two(input: &str) -> Option<impl Debug + Display + num::Integer> {
    let input = input.to_uppercase();
    Some(input.lines().map(|l| solve_entry(&l)).sum::<usize>())
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
