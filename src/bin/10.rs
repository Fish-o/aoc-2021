use itertools::Itertools;
use rayon::prelude::*;
use std::fmt::{Debug, Display};
advent_of_code::solution!(10);

fn reverse_char(c: &char) -> char {
    match c {
        '>' => '<',
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '<' => '>',
        '(' => ')',
        '[' => ']',
        '{' => '}',
        _ => panic!(),
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|l| {
                let mut stack = vec![];
                for c in l.chars() {
                    match c.clone() {
                        '<' | '(' | '[' | '{' => {
                            stack.push(c);
                        }
                        '>' | ')' | ']' | '}' => match stack.pop() {
                            None => {}
                            Some(p) => {
                                if c != reverse_char(&p) {
                                    return Some((c, reverse_char(&p)));
                                }
                            }
                        },
                        _ => {}
                    }
                }
                None
            })
            .map(|e| match e {
                Some(('>', _)) => 25137,
                Some(('}', _)) => 1197,
                Some((']', _)) => 57,
                Some((')', _)) => 3,
                _ => 0,
            })
            .sum::<usize>(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let r = input
        .lines()
        .map(|l| {
            let mut stack = vec![];
            for c in l.chars() {
                match c.clone() {
                    '<' | '(' | '[' | '{' => {
                        stack.push(c);
                    }
                    '>' | ')' | ']' | '}' => match stack.pop() {
                        None => {}
                        Some(p) => {
                            if c != reverse_char(&p) {
                                return None;
                            }
                        }
                    },
                    _ => {}
                }
            }
            Some(stack)
        })
        .filter(|s| s.is_some())
        .map(|s| s.unwrap().into_iter().rev().collect_vec())
        .map(|s| {
            let mut v = 0;
            for c in s {
                let a = match reverse_char(&c) {
                    '>' => 4,
                    '}' => 3,
                    ']' => 2,
                    ')' => 1,
                    _ => continue,
                };
                v *= 5;
                v += a;
            }
            v
        })
        .sorted()
        .collect_vec();
    Some(r.iter().skip(r.len() / 2).next().unwrap().clone())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(26397));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
