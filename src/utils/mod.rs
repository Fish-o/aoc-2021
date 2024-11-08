pub mod tree;
pub mod parsing;

pub fn transform<E: Clone>(d: &Vec<Vec<E>>) -> Vec<Vec<E>> {
    let h = d.len();
    let w = d.first().expect("Empty matrix").len();
    let mut res = Vec::with_capacity(w);
    for x in 0..w{
        let mut col = Vec::with_capacity(h);
        for y in 0..h{
            col.push(d.iter().nth(y).unwrap().iter().nth(x).unwrap().clone());
        }
        res.push(col);
    }
    res
}