use std::{
    collections::HashMap,
    error,
    fmt::{Debug, Display},
    ops::{Add, AddAssign},
    str::FromStr,
};

use itertools::Itertools;
use rayon::collections;
/// https://en.wikipedia.org/wiki/Minkowski_distance

pub enum Metric {
    Chebyshev,
    TODO_Euclidean,
    Taxicab,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos(usize, usize);
impl Pos {
    pub fn from_rc(row: usize, col: usize) -> Self {
        Pos(row, col)
    }
    pub fn from_xy(x: usize, y: usize) -> Self {
        Pos(y, x)
    }
    pub fn get_xy(&self) -> (usize, usize) {
        (self.1, self.0)
    }
    pub fn get_rc(&self) -> (usize, usize) {
        (self.0, self.1)
    }
}
impl Add for Pos {
    type Output = Pos;
    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}
// TODO: Make row_sep and col_sep a split pattern instead of a string
pub struct Matrix<E> {
    row_sep: String,
    col_sep: String,
    data: Vec<Vec<E>>,
}

impl<E: Clone> Matrix<E> {
    pub fn height(&self) -> usize {
        self.data.len()
    }
    pub fn count(&self) -> usize {
        self.height() * self.width()
    }
    pub fn positions(&self) -> Vec<Pos> {
        (0..self.height())
            .flat_map(|r| {
                (0..self.width())
                    .map(|c| Pos::from_rc(r.clone(), c))
                    .collect_vec()
            })
            .collect_vec()
    }
    pub fn cells_mut(&mut self) -> Vec<&mut E> {
        self.data
            .iter_mut()
            .flat_map(|row| row.iter_mut().collect_vec())
            .collect_vec()
    }
    pub fn enumerate(&self) -> Vec<(Pos, &E)> {
        self.data
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .map(|(c, v)| (Pos::from_rc(r, c), v))
                    .collect_vec()
            })
            .collect_vec()
    }
    pub fn width(&self) -> usize {
        if self.height() == 0 {
            0
        } else {
            self.data.first().unwrap().len()
        }
    }
    pub fn get_pos(&self, pos: &Pos) -> Option<&E> {
        Some(self.data.get(pos.0)?.get(pos.1)?)
    }
    pub fn get_pos_mut(&mut self, pos: &Pos) -> Option<&mut E> {
        Some(self.data.get_mut(pos.0)?.get_mut(pos.1)?)
    }

    pub fn touching_positions(&self, pos: &Pos) -> Vec<Pos> {
        let mut r = vec![];
        if pos.0 > 0 {
            r.push(Pos::from_rc(pos.0 - 1, pos.1));
        }
        if pos.0 < self.height() - 1 {
            r.push(Pos::from_rc(pos.0 + 1, pos.1));
        }
        if pos.1 > 0 {
            r.push(Pos::from_rc(pos.0, pos.1 - 1));
        }
        if pos.1 < self.width() - 1 {
            r.push(Pos::from_rc(pos.0, pos.1 + 1));
        }
        r
    }
    pub fn touching_cells(&self, pos: &Pos) -> Vec<&E> {
        self.touching_positions(pos)
            .iter()
            .map(|p| self.get_pos(p).expect("Neighbour does not exist!?"))
            .collect_vec()
    }
    pub fn neighbours_in_range(&self, p: &Pos, metric: &Metric, mut distance: usize) -> Vec<Pos> {
        let mut res = vec![];
        let (r, c) = p.get_rc();
        match metric {
            Metric::Chebyshev => {
                for row in (if r < distance { 0 } else { r - distance })
                    ..=(if r + distance >= self.height() {
                        self.height() - 1
                    } else {
                        r + distance
                    })
                {
                    for col in (if c < distance { 0 } else { c - distance })
                        ..=(if c + distance >= self.width() {
                            self.width() - 1
                        } else {
                            c + distance
                        })
                    {
                        if row != r || col != c {
                            res.push(Pos::from_rc(row, col));
                        }
                    }
                }
            }
            Metric::Taxicab => {
                let h = self.height() as i128;
                let w = self.width() as i128;
                for d in 1..=distance {
                    let mut row: i128 = (r as i128) - (d as i128);
                    let mut col: i128 = c as i128;
                    for (dr, dc) in vec![(-1, 1), (-1, -1), (1, -1), (1, 1)] {
                        for _ in 0..d {
                            row += dr;
                            col += dc;
                            if row < 0 || row >= h || col < 0 || col > w {
                                continue;
                            }
                            res.push(Pos::from_rc(row as usize, col as usize));
                        }
                    }
                }
            }
            Metric::TODO_Euclidean => {
                // TODO: Implement euclidean distance
                todo!()
            }
        }
        res
    }
    pub fn rows(&self) -> Vec<Vec<&E>> {
        self.data
            .iter()
            .map(|r| r.iter().collect_vec())
            .collect_vec()
    }
    pub fn columns(&self) -> Vec<Vec<&E>> {
        if self.data.len() == 0 {
            return vec![];
        }
        let h = self.data.len();
        let w = self
            .data
            .first()
            .expect("Matrix empty while it should not be!?")
            .len();
        let mut res = Vec::with_capacity(w);
        for c in 0..w {
            let mut col = Vec::with_capacity(h);
            for r in 0..h {
                col.push(self.get(r, c))
            }
            res.push(col);
        }
        res
    }
    pub fn get(&self, row: usize, col: usize) -> &E {
        self.data.iter().nth(row).unwrap().iter().nth(col).unwrap()
    }

    /// flood_once denotes if each cell will only flood once to its neighbours, or continously.
    pub fn flood_regions<F>(
        &self,
        seeds: &Vec<Pos>,
        f: F,
        flood_once: bool,
        allow_merging: bool,
    ) -> Vec<Vec<Pos>>
    where
        F: Fn(&Self, &Pos, &Vec<Pos>) -> Vec<Pos>,
    {
        let m = self;
        assert!(seeds.iter().all(|r| m.get_pos(r).is_some()));
        let mut regions = seeds
            .iter()
            .map(|s| (vec![s.clone()], vec![s.clone()]))
            .collect_vec();
        let mut updated = true;
        let mut region_owns: Vec<Vec<Option<usize>>> = vec![vec![None; m.width()]; m.height()];
        seeds
            .iter()
            .enumerate()
            .map(|(i, p)| (i, p.get_rc()))
            .for_each(|(i, (r, c))| {
                *region_owns.get_mut(r).unwrap().get_mut(c).unwrap() = Some(i);
            });

        let mut takeovers = (0..seeds.len()).collect_vec();
        while updated {
            updated = false;
            // (a, b, size)
            // a collides into b, and a has size "size"
            let mut collisions = vec![];

            regions
                .iter_mut()
                .enumerate()
                .for_each(|(i, (to_flood, region))| {
                    let mut new_cells = vec![];
                    for cell in to_flood.iter() {
                        let floods_to = f(&m, cell, &region)
                            .into_iter()
                            .filter(|c| !region.contains(c) && !to_flood.contains(c))
                            .collect_vec();
                        if floods_to.len() > 0 {
                            updated = true;
                        }
                        let mut floods_to = floods_to
                            .into_iter()
                            .filter(|p| {
                                let (r, c) = p.get_rc();
                                let owned_by = region_owns.get_mut(r).unwrap().get_mut(c).unwrap();
                                if owned_by.is_none() {
                                    *owned_by = Some(i);
                                    return true;
                                }
                                let owned_by = owned_by.unwrap();
                                let owned_by = takeovers[owned_by];
                                if owned_by == i {
                                    return false;
                                }
                                collisions.push((i, owned_by, region.len()));
                                return false;
                            })
                            .collect_vec();
                        new_cells.append(&mut floods_to);
                    }
                    region.append(to_flood);
                    if flood_once {
                        std::mem::swap(to_flood, &mut new_cells);
                    }
                });

            collisions.sort_by_key(|c| c.2);
            for (a, b, _) in collisions {
                let ai = *takeovers.get(a).unwrap();
                let bi = *takeovers.get(b).unwrap();
                if ai == bi {
                    continue;
                }
                let b = regions.get_mut(bi).unwrap();
                let mut c = (vec![], vec![]);
                std::mem::swap(b, &mut c);
                let a = regions.get_mut(ai).unwrap();
                a.0.append(&mut c.0);
                a.1.append(&mut c.1);

                a.0.sort();
                a.0.dedup();

                a.1.sort();
                a.1.dedup();

                takeovers.iter_mut().for_each(|f| {
                    if f == &bi {
                        *f = ai;
                    }
                });
            }
        }
        regions
            .into_iter()
            .filter(|(_, b)| !b.is_empty())
            .map(|(_, r)| r)
            .collect_vec()
    }
}
impl Matrix<String> {
    pub fn from_str(input: &str, row_sep: &str, col_sep: &str) -> Self {
        Matrix {
            row_sep: row_sep.to_string(),
            col_sep: col_sep.to_string(),
            data: input
                .trim()
                .split(row_sep)
                .map(|r| {
                    if col_sep.is_empty() {
                        r.chars().map(|c| c.to_string()).collect_vec()
                    } else {
                        r.split(col_sep).map(|s| s.to_owned()).collect_vec()
                    }
                })
                .collect_vec(),
        }
    }
}

impl<S: AsRef<str>> Matrix<S> {
    pub fn parse<F>(&self) -> Result<Matrix<F>, <F as FromStr>::Err>
    where
        F: FromStr,
    {
        Ok(Matrix {
            row_sep: self.row_sep.clone(),
            col_sep: self.col_sep.clone(),
            data: self
                .data
                .iter()
                .map(|r| r.iter().map(|c| F::from_str(c.as_ref())).collect())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl<E> std::fmt::Display for Matrix<E>
where
    E: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .iter()
                .map(|r| r.iter().map(|c| c.to_string()).join(&self.col_sep))
                .join(&self.row_sep)
        )
    }
}

impl<E> std::fmt::Debug for Matrix<E>
where
    E: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .iter()
                .map(|r| r.iter().map(|c| format!("{:?}", c)).join(&self.col_sep))
                .join(&self.row_sep)
        )
    }
}

impl<E> Clone for Matrix<E>
where
    E: Clone,
{
    fn clone(&self) -> Self {
        Self {
            row_sep: self.row_sep.clone(),
            col_sep: self.col_sep.clone(),
            data: self.data.clone(),
        }
    }
}

#[test]
pub fn test() {
    let input1 = "abc\ndef\nhij";
    let m1: Matrix<char> = Matrix::from_str(input1, "\n", "").parse().unwrap();
    println!("{}", m1);

    let input2 = "5 3 1\n6 2 9\n8 0 4";
    let m2: Matrix<u32> = Matrix::from_str(input2, "\n", " ").parse().unwrap();
    println!("{}", m2);

    let input3 = "-5 19 2\n-61 9 -6\n8 -3 4";
    let m3: Matrix<i64> = Matrix::from_str(input3, "\n", " ").parse().unwrap();
    println!("{}", m3);
}

pub fn transform<E: Clone>(d: &Vec<Vec<E>>) -> Vec<Vec<E>> {
    let h = d.len();
    let w = d.first().expect("Empty matrix").len();
    let mut res = Vec::with_capacity(w);
    for x in 0..w {
        let mut col = Vec::with_capacity(h);
        for y in 0..h {
            col.push(d.iter().nth(y).unwrap().iter().nth(x).unwrap().clone());
        }
        res.push(col);
    }
    res
}
