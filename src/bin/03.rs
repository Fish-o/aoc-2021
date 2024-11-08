use itertools::Itertools;
use rayon::prelude::*;
use std::fmt::{Debug, Display};
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<impl Debug + Display + num::Integer> {
    let mut bits = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for line in input.lines() {
        for i in 0..12 {
            if line.chars().nth(i).unwrap() == '0' {
                bits[i] += 1;
            } else {
                bits[i] -= 1;
            }
        }
    }
    println!("{:?}", bits);
    let gamma = bits
        .iter()
        .map(|v| if v > &0 { true } else { false })
        .rev()
        .enumerate()
        .fold(
            0i32,
            |r, (i, v)| if v { r + (2i32.pow(i as u32)) } else { r },
        );
    let delta = bits
        .iter()
        .map(|v| if v > &0 { true } else { false })
        .rev()
        .enumerate()
        .fold(
            0i32,
            |r, (i, v)| if !v { r + (2i32.pow(i as u32)) } else { r },
        );

    Some(gamma * delta)
}

pub fn part_two(input: &str) -> Option<impl Debug + Display + num::Integer> {
    let mut bit_set: bool = false;
    let mut o_gen_rating = input.lines().collect::<Vec<_>>();
    let mut n = 0;
    while o_gen_rating.len() > 1 {
        let o2_gen_rating = o_gen_rating
            .iter()
            .map(|f| f.chars().nth(n).unwrap())
            .fold(0, |r, f| if matches!(f, '1') { r + 1 } else { r - 1 });
        println!("{}",o2_gen_rating); 
        let c = if o2_gen_rating >= 0 { '1' } else { '0' };
        o_gen_rating = o_gen_rating
            .clone()
            .iter()
            .filter(|f| f.chars().nth(n).unwrap() == c)
            .cloned()
            .collect_vec();
        println!("{:?}", o_gen_rating);
        n += 1;
    }
    let o_gen_rating = o_gen_rating.first().unwrap();
    
    let mut co_gen_rating = input.lines().collect::<Vec<_>>();
    let mut n = 0;
    while co_gen_rating.len() > 1 {
        let o2_gen_rating = co_gen_rating
            .iter()
            .map(|f| f.chars().nth(n).unwrap())
            .fold(0, |r, f| if matches!(f, '1') { r + 1 } else { r - 1 });
        let c = if o2_gen_rating >= 0 { '0' } else { '1' };
        co_gen_rating = co_gen_rating
            .clone()
            .iter()
            .filter(|f| f.chars().nth(n).unwrap() == c)
            .cloned()
            .collect_vec();
        n += 1;
    }
    
    let c_gen_rating = co_gen_rating.first().unwrap();
    let res = i32::from_str_radix(&o_gen_rating, 2).unwrap() * i32::from_str_radix(&c_gen_rating, 2).unwrap();
    Some(res)
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
