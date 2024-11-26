use itertools::Itertools;
use num::Integer;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
    ops::AddAssign,
};
advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let (s, p) = input.trim().split_once("\n\n").unwrap();
    let mut s = s.trim().chars().collect_vec();
    let mut rules = HashMap::new();
    for line in p.trim().lines() {
        let (p, r) = line.trim().split_once(" -> ").unwrap();
        rules.insert(
            p.chars().collect_tuple::<(_, _)>().unwrap(),
            r.chars().next().unwrap(),
        );
    }
    for _ in 0..10 {
        s = s
            .into_iter()
            .tuple_windows()
            .enumerate()
            .fold(vec![], |mut r, (i, (a, b))| {
                if i == 0 {
                    r.push(a);
                }
                match rules.get(&(a, b)) {
                    Some(c) => r.push(*c),
                    _ => {}
                }
                r.push(b);
                r
            });
    }
    let mut counts = vec![0; 26];
    s.iter().for_each(|c| {
        *(counts.get_mut(*c as usize - 'A' as usize).unwrap()) += 1;
    });
    counts.sort();
    Some(counts.last().unwrap() - counts.iter().filter(|e| **e != 0).next().unwrap())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (s, p) = input.trim().split_once("\n\n").unwrap();
    let mut rules = HashMap::new();
    for line in p.trim().lines() {
        let (p, r) = line.trim().split_once(" -> ").unwrap();
        let r = r.chars().next().unwrap();
        let t = p.chars().collect_tuple::<(_, _)>().unwrap();
        let r = ((t.0, r), (r, t.1));
        rules.insert(t, r);
    }

    let mut pairs: HashMap<(char, char), usize> = HashMap::new();
    s.chars().tuple_windows::<(_, _)>().for_each(|t| {
        pairs.entry(t).or_insert(0).inc();
    });
    pairs.entry((' ', s.chars().next().unwrap())).or_insert(1);
    pairs.entry((s.chars().last().unwrap(), ' ')).or_insert(1);
    for _ in 0..40 {
        let mut np = HashMap::new();
        for (pair, c) in pairs {
            let r = rules.get(&pair);
            match r {
                None => np.entry(pair).or_insert(0).add_assign(c),
                Some((a, b)) => {
                    np.entry(*a).or_insert(0).add_assign(c);
                    np.entry(*b).or_insert(0).add_assign(c);
                }
            }
        }
        pairs = np;
    }
    let mut counts = vec![0; 26];
    pairs.iter().for_each(|(k, c)| {
        if k.0 != ' ' {
            *(counts.get_mut(k.0 as usize - 'A' as usize).unwrap()) += c;
        }
        if k.1 != ' ' {
            *(counts.get_mut(k.1 as usize - 'A' as usize).unwrap()) += c;
        }
    });
    let mut counts = counts.into_iter().map(|c| c / 2).collect_vec();
    counts.sort();
    Some(counts.last().unwrap() - counts.iter().filter(|e| **e != 0).next().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1588));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1588));
    }
}
