use advent_of_code::utils::matrix::{Matrix, Pos};
use itertools::Itertools;
use petgraph::{algo::dijkstra, visit::EdgeRef};
use rayon::prelude::*;
use std::fmt::{Debug, Display};
advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<usize> {
    let m = Matrix::from_ugrid(input);
    let (i_l, g) = m.gen_graph(&vec![Pos::from_rc(0, 0)], |m, p| {
        m.touching_positions(p)
            .into_iter()
            .map(|p| (p.clone(), m.get_pos(&p).unwrap().clone()))
            .collect_vec()
    });
    let goal = i_l
        .get(&Pos::from_rc(m.height() - 1, m.width() - 1))
        .unwrap()
        .clone();

    let l = dijkstra(
        &g,
        i_l.get(&Pos::from_rc(0, 0)).unwrap().clone(),
        Some(goal.clone()),
        |e| *e.weight(),
    );
    Some(l.get(&goal).unwrap().clone())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut m = Matrix::from_ugrid(input);
    let w = m.width();
    let h = m.height();
    for _ in 0..4 {
        for c_i in (m.width() - w)..(m.width()) {
            let col = m.col(c_i).unwrap();
            m.push_col(
                col.into_iter()
                    .map(|v| if *v == 9 { 1 } else { *v + 1 })
                    .collect_vec(),
            );
        }
    }
    for _ in 0..4 {
        for r_i in (m.height() - h)..(m.height()) {
            let row = m.row(r_i).unwrap();
            m.push_row(
                row.into_iter()
                    .map(|v| if *v == 9 { 1 } else { *v + 1 })
                    .collect_vec(),
            );
        }
    }
    let (i_l, g) = m.gen_graph(&vec![Pos::from_rc(0, 0)], |m, p| {
        m.touching_positions(p)
            .into_iter()
            .map(|p| (p.clone(), m.get_pos(&p).unwrap().clone()))
            .collect_vec()
    });
    let goal = i_l
        .get(&Pos::from_rc(m.height() - 1, m.width() - 1))
        .unwrap()
        .clone();
    let l = dijkstra(
        &g,
        i_l.get(&Pos::from_rc(0, 0)).unwrap().clone(),
        Some(goal.clone()),
        |e| *e.weight(),
    );
    Some(l.get(&goal).unwrap().clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(315));
    }
}
