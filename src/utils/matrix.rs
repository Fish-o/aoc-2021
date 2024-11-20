use std::{
    error,
    fmt::{Debug, Display},
    ops::{Add, AddAssign},
    str::FromStr,
};

use itertools::Itertools;
#[derive(Clone, Debug, PartialEq, Eq)]
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
    pub fn neighbour_positions(&self, pos: &Pos) -> Vec<Pos> {
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
    pub fn neighbours(&self, pos: &Pos) -> Vec<&E> {
        self.neighbour_positions(pos)
            .iter()
            .map(|p| self.get_pos(p).expect("Neighbour does not exist!?"))
            .collect_vec()
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
    /// Regions are not merged, and may overlap
    pub fn flood_regions<F>(&self, seeds: &Vec<Pos>, f: F, flood_once: bool) -> Vec<Vec<Pos>>
    where
        F: Fn(&Self, &Pos, &Vec<Pos>) -> Vec<Pos>,
    {
        let m = self;
        assert!(seeds.iter().all(|r| m.get_pos(r).is_some()));
        let mut regions = seeds
            .iter()
            .map(|s| (vec![s.clone()], vec![s.clone()]))
            .collect_vec();
        let mut updated = false;
        while !updated {
            updated = false;
            regions.iter_mut().for_each(|(to_flood, region)| {
                let mut new_cells = vec![];
                for cell in to_flood.iter() {
                    let mut floods_to = f(&m, cell, &region)
                        .into_iter()
                        .filter(|c| !region.contains(c))
                        .collect_vec();
                    if floods_to.len() > 0 {
                        updated = true;
                    }
                    new_cells.append(&mut floods_to);
                }
                region.append(to_flood);
                if flood_once {
                    std::mem::swap(to_flood, &mut new_cells);
                }
            });
        }
        regions.into_iter().map(|(_, r)| r).collect_vec()
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
