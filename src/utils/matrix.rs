use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use itertools::Itertools;
// TODO: Make row_sep and col_sep a split pattern instead of a string
pub struct Matrix<E> {
    row_sep: String,
    col_sep: String,
    data: Vec<Vec<E>>,
}

impl<E> Matrix<E> {
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
}
impl Matrix<String> {
    pub fn from_str(input: &str, row_sep: &str, col_sep: &str) -> Self {
        Matrix {
            row_sep: row_sep.to_string(),
            col_sep: col_sep.to_string(),
            data: input
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
