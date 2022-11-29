//! The Math crate of the project

use std::{clone::Clone, fmt::Display, collections::LinkedList};

/// A structure meant to be passed as a generic parameter to other functions.
/// it is meant to ensapsulate all the information related to 
/// - the types used 
/// (notice that the types might be anything, you could for example have sequences of strings,
/// all you have to do is to define a MetricSpace and the rest of the code is guaranteed to work), 
/// - the distance constants 
/// - the usual values for zero and infinity.
/// This makes it possible to use multiple MetricSpaces at once in the same program
/// and makes the distance functions reusable with a large variety of sequence types.
pub trait MetricSpace {
    type Item: Copy + Display;
    type Cost: Ord + std::ops::Add<Output = Self::Cost> + Copy + std::fmt::Debug;

    const ZEROCOST: Self::Cost;
    const INFCOST: Self::Cost;
    const GAP: Self::Item;
    const DEL: Self::Cost;
    const INS: Self::Cost;
    fn sub(a: Self::Item, b: Self::Item) -> Self::Cost;
}
#[derive(Debug, PartialEq, Eq)]
pub struct Align<M: MetricSpace>(Vec<M::Item>, Vec<M::Item>);

impl<M: MetricSpace> Display for Align<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n| ")?;
        for a in self.0.iter() {
            write!(f, "{} ", a)?;
        }
        write!(f, "\n| ")?;
        for a in self.1.iter() {
            write!(f, "{} ", a)?;
        }
        Ok(())
    }
}

/// Calculate the cost of the alignment (x, y) passed as parameter
pub fn cout_align<M>(x: &[M::Item], y: &[M::Item]) -> M::Cost
where M: MetricSpace, <M as MetricSpace>::Item: PartialEq + std::fmt::Debug
{
    assert_eq!(x.len(), y.len(), "{:?} : {:?}", x, y);
    if x.is_empty() && y.is_empty() { M::ZEROCOST }
    else {
        cout_align::<M>(&x[1..], &y[1..]) + 
        match (x[0], y[0]) {
            (x, y) if x == M::GAP && y == M::GAP => M::INS + M::DEL,
            (x, _) if x == M::GAP => M::INS,
            (_, y) if y == M::GAP => M::DEL,
            (x, y) => M::sub(x, y)
        } 
    }
}

/// Calculate distance between sequences x and y in the MetricSpace M
/// O(exp(n)) time and memory
pub fn dist_naif<M>(x: &[M::Item], y: &[M::Item]) -> M::Cost
where M: MetricSpace
{
    dist_naif_rec::<M, _>(x.iter().copied(), y.iter().copied(), M::ZEROCOST, M::INFCOST)
}

/// auxiliary function for dist_naif
pub fn dist_naif_rec<T, I>(mut xi: I, mut yi: I, c: T::Cost, mut dist: T::Cost) -> T::Cost
where T: MetricSpace, I: Iterator<Item = T::Item> + Clone
{
    let (xo, yo) = (xi.clone(), yi.clone());
    let m = (xi.next(), yi.next());
    if let (None, None) = m { return c.min(dist); }
    if let (Some(xj), Some(yj)) = m { dist = dist_naif_rec::<T, I>(xi.clone(), yi.clone(), c + T::sub(xj, yj), dist); }
    if let (Some(_), _) = m { dist = dist_naif_rec::<T, I>(xi, yo, c + T::DEL, dist); }
    if let (_, Some(_)) = m { dist = dist_naif_rec::<T, I>(xo, yi, c + T::INS, dist); }
    dist
}

/// Compute the 2D dynamic-programming table for the sequences x and y
pub fn dist_dp_full<M>(x: &[M::Item], y: &[M::Item]) -> Vec<Vec<M::Cost>>
where M: MetricSpace, 
{
    use std::cmp::min;

    let n = x.len() + 1;
    let m = y.len() + 1;
    let mut dp = vec![vec![M::ZEROCOST; m]; n];
    for i in 1..n {
        dp[i][0] = dp[i-1][0] + M::DEL;
    }
    for j in 1..m {
        dp[0][j] = dp[0][j-1] + M::INS;
    }
    for i in 1..n {
        for j in 1..m {
            dp[i][j] = min(
                dp[i-1][j-1] + M::sub(x[i-1], y[j-1]), min(
                dp[i][j-1] + M::INS,
                dp[i-1][j] + M::DEL
            ))
        }
    }
    dp
}

/// Compute distance using a 2D table O(n^2) time O(n^2) memory 
pub fn dist_1<M>(x: &[M::Item], y: &[M::Item]) -> M::Cost 
where M: MetricSpace
{
    let dp = dist_dp_full::<M>(x, y);
    dp[x.len()][y.len()]
}

/// Compute the optimal alignment using a 2D table O(n^2) time and memory
pub fn sol_1<M>(x: &[M::Item], y: &[M::Item]) -> Align<M>
where M: MetricSpace
{
    let t = dist_dp_full::<M>(x, y);
    sol_1_tab(x, y, t.as_slice())
}

/// Same as sol_1 but you pass in the table manually
pub fn sol_1_tab<M>(x: &[M::Item], y: &[M::Item], t: &[Vec<M::Cost>]) -> Align<M>
where M: MetricSpace
{
    let n = x.len();
    let m = y.len();
    assert_eq!(n + 1, t.len());
    assert_eq!(m + 1, t[0].len());
    let mut xb = vec![];
    let mut yb = vec![];

    let mut i = n;
    let mut j = m;
    while i > 0 && j > 0 {
        if t[i][j] == t[i-1][j-1] + M::sub(x[i-1], y[j-1]) {
            xb.push(x[i-1]);
            yb.push(y[j-1]);
            i -= 1;
            j -= 1;
        } else if t[i][j] == t[i][j-1] + M::INS {
            xb.push(M::GAP);
            yb.push(y[j-1]);
            j -= 1;
        } else {
            // assert_eq!(if t[i][j], t[i-1][j] + M::DEL);
            xb.push(x[i-1]);
            yb.push(M::GAP);
            i -= 1;
        }
    }
    while i > 0 {
        xb.push(x[i-1]);
        yb.push(M::GAP);
        i -= 1;
    }
    while j > 0 {
        xb.push(M::GAP);
        yb.push(y[j-1]);
        j -= 1;
    }
    xb.reverse();
    yb.reverse();
    Align(xb, yb)
}

/// Calculate the optimal alignment and the distance between the sequences at once
pub fn prog_dyn<M>(x: &[M::Item], y: &[M::Item]) -> (M::Cost, Align<M>)
where M: MetricSpace
{
    let dp = dist_dp_full::<M>(x, y);
    (dp[x.len()][y.len()], sol_1_tab::<M>(x, y, dp.as_slice()))
}

/// Calculate the distance between two sequences O(n^2) time O(n) memory
pub fn dist_2<M>(x: &[M::Item], y: &[M::Item]) -> M::Cost 
where M: MetricSpace
{
    use std::cmp::min;

    let n = x.len() + 1;
    let m = y.len() + 1;
    let mut dp = vec![vec![M::ZEROCOST; m]; 2];

    for j in 1..m {
        dp[0][j] = dp[0][j-1] + M::INS;
    }
    for i in 1..n {
        dp[1][0] = dp[0][0] + M::DEL;
        for j in 1..m {
            dp[1][j] = min(
                dp[0][j-1] + M::sub(x[i-1], y[j-1]),
                min(
                    dp[1][j-1] + M::INS,
                    dp[0][j] + M::DEL
                )
            )
        }
        dp.swap(0, 1);
    }
    dp[0][y.len()]
}

/// Calculate the optimal cutting point in sequence y for the corresponding 
/// cutting point |x|/2 in sequence x 
pub fn coupure<M>(x: &[M::Item], y: &[M::Item]) -> usize 
where M: MetricSpace
{
    use std::cmp::min;
    let n = x.len() + 1;
    let m = y.len() + 1;

    let i_star = x.len()/2;
    let mut t = vec![vec![M::ZEROCOST; m]; 2];
    let mut q = vec![vec![0; m]; 2];

    t[0][0] = M::ZEROCOST;
    q[0][0] = 0;

    for j in 1..m {
        t[0][j] = t[0][j-1] + M::INS;
        q[0][j] = j;
    }

    for i in 1..n {
        t[1][0] = t[0][0] + M::DEL;
        for j in 1..m {
            let op1 = t[0][j-1] + M::sub(x[i-1], y[j-1]);
            let op2 = t[0][j] + M::DEL;
            let op3 = t[1][j-1] + M::INS;

            t[1][j] = min(op1, min(op2, op3));

            if i <= i_star          { continue; }
            if t[1][j] == op1       { q[1][j] = q[0][j-1]; }
            else if t[1][j] == op2  { q[1][j] = q[0][j]; }
            else                    { q[1][j] = q[1][j-1]; }
        }
        t.swap(0, 1);
        if i > i_star { q.swap(0, 1); }
    }

    q[0][y.len()]
}

/// Generate a word composed of gaps of length n
pub fn mot_gaps<M>(n: usize) -> LinkedList<M::Item>
where M: MetricSpace
{
    let mut ret = LinkedList::from([]);
    for _ in 0..n { ret.push_back(M::GAP); }
    ret
}


/// Remove all gaps from the sequence
pub fn rm_gaps<M>(a: Vec<M::Item>) -> Vec<M::Item>
where M: MetricSpace,  <M as MetricSpace>::Item: PartialEq
{
    a.into_iter().filter(|&x| x != M::GAP).collect::<Vec<_>>()
}

/// Align a letter with a word in the optimal way. O(n) time
pub fn align_lettre_mot<M>(x: M::Item, y: &[M::Item]) -> (LinkedList<M::Item>, LinkedList<M::Item>)
where M: MetricSpace
{
    let (i, _) = y
        .iter()
        .enumerate()
        .min_by_key(|&(_, &yk)| M::sub(x, yk))
        .expect("y is empty!");
    let mut xb = mot_gaps::<M>(i);
    xb.push_back(x);
    xb.append(&mut mot_gaps::<M>(y.len() - 1 - i));
    (xb, LinkedList::from_iter(y.iter().copied()))
} 

/// Auxiliary function for sol_2 O(n^2) time and O((n+m)log n) memory
pub fn sol_2_ll<M>(x: &[M::Item], y: &[M::Item]) -> (LinkedList<M::Item>, LinkedList<M::Item>) 
where M: MetricSpace
{
    match (x.len(), y.len()) {
        (0, _) => (mot_gaps::<M>(y.len()), LinkedList::from_iter(y.iter().copied())),
        (_, 0) => (LinkedList::from_iter(x.iter().copied()), mot_gaps::<M>(x.len())),
        (1, _) => align_lettre_mot::<M>(x[0], y),
        (_, _) => {
            let i = x.len()/2;
            let j = coupure::<M>(x, y);
    
            let (mut x1, mut y1) = sol_2_ll::<M>(&x[0..i], &y[0..j]);
            let (mut x2, mut y2) = sol_2_ll::<M>(&x[i..], &y[j..]);
    
            x1.append(&mut x2);
            y1.append(&mut y2);
    
            (x1, y1)
        }
    }
}

/// Compute the alignement of two sequences in O(n^2) time and O((n+m)log n) memory
pub fn sol_2<M>(x: &[M::Item], y: &[M::Item]) -> Align<M> 
where M: MetricSpace
{
    let (a, b) = sol_2_ll::<M>(x, y);
    Align(Vec::from_iter(a.into_iter()), Vec::from_iter(b.into_iter()))
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use std::fmt::Display;

    use crate::dna::{DnaMetricSpace as Dms, DnaBlock, Dna};
    use crate::io::{read_test_inst, read_test_insts_all};
    use crate::math::{sol_1_tab, dist_dp_full, rm_gaps};

    use super::{Align, cout_align};

    fn test_dist_3<F>(f: F, name: &str) // manual tests
    where F: Fn(&Vec<Dna>, &Vec<Dna>) -> u64
    {
        let filenames = &[
            (10, "Inst_0000010_44.adn"),
            (8, "Inst_0000010_7.adn"),
            (2, "Inst_0000010_8.adn")
        ];

        let testcases = filenames
            .iter()
            .map(|(r, p)| (r, read_test_inst(p).expect("cannot read instance!")));

        for (result, DnaBlock(l, r)) in testcases {
            let d = f(&l, &r);
            assert_eq!(d, *result, 
                "result for {}({:?}, {:?}) should be {} but {} was given instead!", name, l, r, *result, d);
        }
    }

    fn test_against<F, F2, F3, T>(f: F, f2: F2, comp: F3, name: &str)
    where F: Fn(&Vec<Dna>, &Vec<Dna>) -> T, F2: Fn(&Vec<Dna>, &Vec<Dna>) -> T, F3: Fn(&T, &T) -> bool, T: Eq + Display + Debug
    {
        let testcases = read_test_insts_all()
            .take_while(|&(size, _)| size <= 20)
            .map(|(_, b)| b); // this will read the files one by one

        for DnaBlock(l, r) in testcases {
            let a = f(&l, &r);
            let b = f2(&l, &r);
            assert!(comp(&a, &b), 
                "result mismatch! : {}({:?}, {:?}) = {} != {} = reference(..., ...)", name, l, r, a, b);
        }
    }

    #[test]
    fn cout_align_dna(){
        use Dna::*;
        assert_eq!(cout_align::<Dms>(&[A, T, Gap, A, C], &[Gap, T, G, A, C]), 4);
    }

    #[test]
    fn dist_naif_dna(){
        test_dist_3(|l: &Vec<Dna>, r: &Vec<Dna>| -> u64 {
            super::dist_naif::<Dms>(l, r)
        }, "dist_naif");
    }

    #[test]
    fn dist_1_dna(){
        test_dist_3(|l: &Vec<Dna>, r: &Vec<Dna>| -> u64 {
            super::dist_1::<Dms>(l, r)
        }, "dist_1");
    }

    #[test]
    fn dist_2_dna(){
        test_dist_3(|l: &Vec<Dna>, r: &Vec<Dna>| -> u64 {
            super::dist_2::<Dms>(l, r)
        }, "dist_2");

        test_against(|l: &Vec<Dna>, r: &Vec<Dna>| -> u64 {
            super::dist_2::<Dms>(l, r)
        }, |l: &Vec<Dna>, r: &Vec<Dna>| -> u64 {
            super::dist_1::<Dms>(l, r)
        }, |a, b| a == b, "dist_2")
    }

    #[test]
    fn sol1_2(){ // sanity tests
        use super::{dist_2, sol_1, sol_2, rm_gaps};

        let testcases = read_test_insts_all() // this does not read all files at once
            .take_while(|&(size, _)| size <= 500)
            .map(|(_, b)| b)
            .map(|b| (dist_2::<Dms>(b.0.as_slice(), b.1.as_slice()), b))
            .map(|(a, b)| (b, a));

        for (DnaBlock(l, r), d) in testcases {
            let al = sol_1::<Dms>(l.as_slice(), r.as_slice());
            assert_eq!(rm_gaps::<Dms>(al.0.clone()), *l, "some letters 🍪 got eaten in sol_1");
            assert_eq!(rm_gaps::<Dms>(al.1.clone()), *r, "some letters 🍪 got eaten in sol_1");
            let x = cout_align::<Dms>(al.0.as_slice(), al.1.as_slice());
            assert_eq!(x, d);
            
            ///////////////
            
            let al = sol_2::<Dms>(l.as_slice(), r.as_slice());
            assert_eq!(rm_gaps::<Dms>(al.0.clone()), *l, "some letters 🍪 got eaten in sol_2");
            assert_eq!(rm_gaps::<Dms>(al.1.clone()), *r, "some letters 🍪 got eaten in sol_2");
            let x = cout_align::<Dms>(al.0.as_slice(), al.1.as_slice());
            assert_eq!(x, d);
        }
    }

    #[test]
    fn sol1(){ // additional manual tests
        use Dna::*;
        let filenames = &[
            (Align(vec![T, A, T, A, T, G, A, G, T, C], vec![T, A, T, Gap, T, Gap, Gap, Gap, T, Gap]), "Inst_0000010_44.adn"),
        ];

        let testcases = filenames
            .iter()
            .map(|(t, p)| (t, read_test_inst(p).expect("couldn't read instance!")));

        for (result, DnaBlock(l, r)) in testcases {
            let t = dist_dp_full::<Dms>(l.as_slice(), r.as_slice());
            for line in &t {
                println!("tableau: {:?}", line);
            }
            let d = sol_1_tab::<Dms>(l.as_slice(), r.as_slice(), t.as_slice());

            assert_eq!(rm_gaps::<Dms>(d.0.clone()), *l, "some letters 🍪 got eaten in sol_1");
            assert_eq!(rm_gaps::<Dms>(d.1.clone()), *r, "some letters 🍪 got eaten in sol_1");

            assert_eq!(d, *result, 
                "result for sol_1({:?}, {:?}) should be {} but {} was given instead!", l, r, *result, d);
        }
    }

    #[test]
    fn sol2(){
        use super::rm_gaps;
        test_against(|l: &Vec<Dna>, r: &Vec<Dna>| -> Align<Dms>  {
            let Align::<Dms>(a, b) = super::sol_2(l, r);
            assert_eq!(rm_gaps::<Dms>(a.clone()), *l, "some letters 🍪 got eaten in sol_2");
            assert_eq!(rm_gaps::<Dms>(b.clone()), *r, "some letters 🍪 got eaten in sol_2");
            Align(a, b)
        }, |l: &Vec<Dna>, r: &Vec<Dna>| -> Align<Dms> {
            let Align::<Dms>(a, b) = super::sol_1(l, r);
            assert_eq!(rm_gaps::<Dms>(a.clone()), *l, "some letters 🍪 got eaten in sol_1");
            assert_eq!(rm_gaps::<Dms>(b.clone()), *r, "some letters 🍪 got eaten in sol_1");
            Align(a, b)
        }, |a, b| {
            let l = super::cout_align::<Dms>(a.0.as_slice(), a.1.as_slice());
            let r = super::cout_align::<Dms>(b.0.as_slice(), b.1.as_slice());
            l == r
        }, "sol_2")
    }

    #[test]
    fn prog_dyn_dna(){
        // tests for prog_dyn are not needed since we already test sol_1 and dist_dp_full in other tests.
    }
}