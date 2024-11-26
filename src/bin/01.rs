use advent_of_code::utils::*;
use itertools::Itertools;
use rayon::prelude::*;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        parsing::to_int_vec(input)
            .windows(2)
            .fold(0i32, |r, p| if p[0] < p[1] { r + 1 } else { r }),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parsing::to_int_vec(input)
            .windows(3)
            .map(|w| w[0] + w[1] + w[2])
            .tuple_windows()
            .fold(0u32, |r, w: (i64, i64)| if w.0 < w.1 { r + 1 } else { r }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
}
