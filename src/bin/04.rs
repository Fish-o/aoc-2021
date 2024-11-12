use advent_of_code::utils::matrix::transform;
use itertools::Itertools;
use rayon::prelude::*;
use std::fmt::{Debug, Display};
advent_of_code::solution!(4);
#[derive(Debug, Clone)]
struct BingoCard {
    board: Vec<Vec<Option<usize>>>,
}

impl BingoCard {
    pub fn from(string: &str) -> Self {
        let lines = string.lines();
        BingoCard {
            board: lines
                .map(|l| {
                    l.split_whitespace()
                        .map(|n| Some(n.parse().unwrap()))
                        .collect::<Vec<_>>()
                })
                .collect_vec(),
        }
    }

    pub fn mark_number(&mut self, number: usize) {
        self.board.iter_mut().for_each(|r| {
            r.iter_mut().for_each(|c| match c {
                Some(n) if *n == number => {
                    *c = None;
                }
                _ => {}
            })
        });
    }
    pub fn val(&self) -> usize {
        self.board.iter().fold(0, |f, r| {
            f + r.iter().fold(0, |r, c| match c {
                Some(v) => v + r,
                _ => r,
            })
        })
    }

    pub fn has_row(&self) -> bool {
        self.board.iter().any(|r| r.iter().all(|f| f.is_none()))
    }

    pub fn has_col(&self) -> bool {
        transform(&self.board)
            .iter()
            .any(|r| r.iter().all(|f| f.is_none()))
    }
}

pub fn part_one(input: &str) -> Option<impl Debug + Display + num::Integer> {
    let parts = input.split("\n\n").collect_vec();
    let first = parts.first().unwrap();
    let other = parts.iter().skip(1).collect_vec();
    let mut cards = other.iter().map(|s| BingoCard::from(s)).collect_vec();
    let mut won_card = None;
    let mut i = 0;
    for instruction in first.trim().split(",") {
        i = instruction.parse().unwrap();
        cards.iter_mut().for_each(|c| c.mark_number(i));
        won_card = cards.iter().find(|c| c.has_col() || c.has_row());
        if won_card.is_some() {
            break;
        }
    }
    Some(won_card.unwrap().val() * i)
}

pub fn part_two(input: &str) -> Option<impl Debug + Display + num::Integer> {
    let parts = input.split("\n\n").collect_vec();
    let first = parts.first().unwrap();
    let other = parts.iter().skip(1).collect_vec();
    let mut cards = other.iter().map(|s| BingoCard::from(s)).collect_vec();
    let mut i = 0;
    for instruction in first.trim().split(",") {
        i = instruction.parse().unwrap();
        cards.iter_mut().for_each(|c| c.mark_number(i));
        let new_cards = cards
            .clone()
            .iter()
            .filter(|c| !(c.has_col() || c.has_row()))
            .cloned()
            .collect_vec();
        if new_cards.len() == 0 {
            break;
        }
        cards = new_cards
    }
    Some(cards.first().unwrap().val() * i)
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
