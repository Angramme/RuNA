//! The Math crate of the project

use std::clone::Clone;

pub trait MetricSpace {
    type Item: Copy;
    type Cost: Ord + std::ops::Add<Output = Self::Cost> + Copy;

    const ZEROCOST: Self::Cost;
    const INFCOST: Self::Cost;
    const DEL: Self::Cost;
    const INS: Self::Cost;
    fn sub(a: Self::Item, b: Self::Item) -> Self::Cost;
}

/// Calculate distance between sequences x and y in the MetricSpace M
pub fn dist_naif<M>(x: &[M::Item], y: &[M::Item]) -> M::Cost
where M: MetricSpace
{
    dist_naif_rec::<M, _>(x.iter().copied(), y.iter().copied(), M::ZEROCOST, M::INFCOST)
}

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

pub fn dist_1<M>(x: &[M::Item], y: &[M::Item]) -> M::Cost 
where M: MetricSpace, 
{
    use std::cmp::min;

    let n = x.len() + 1;
    let m = y.len() + 1;
    let mut dp = vec![vec![M::ZEROCOST; m]; n];
    for i in 1..n {
        dp[i][0] = dp[i-1][0] + M::INS;
    }
    for j in 1..m {
        dp[0][j] = dp[0][j-1] + M::DEL;
    }
    for i in 1..n {
        for j in 1..m {
            dp[i][j] = min(
                dp[i-1][j-1] + M::sub(x[i-1], y[j-1]),
                min(
                    dp[i][j-1] + M::DEL,
                    dp[i-1][j] + M::INS
                )
            )
        }
    }
    dp[x.len()][y.len()]
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use std::env;

    use crate::dna::{DnaMetricSpace, DnaBlock, Dna};

    fn test_dist_3<F>(f: F, name: &str)
    where F: Fn(&Vec<Dna>, &Vec<Dna>) -> u64
    {
        let gdata = env::var("GENOME_DATA").expect("GENOME_DATA environnement variable cannot be found!");
        let filenames = &[
            (10, "Inst_0000010_44.adn"),
            (8, "Inst_0000010_7.adn"),
            (2, "Inst_0000010_8.adn")
        ];

        let testcases = filenames
            .iter()
            .map(|(t, p)| (t, gdata.clone() + "/" + p))
            .map(|(t, p)| (t, read_to_string(p).expect("cannot read file!")))
            .map(|(t, s)| (t, s.parse::<DnaBlock>().expect("cannot parse file: {}")));

        for (result, DnaBlock(l, r)) in testcases {
            let d = f(&l, &r);
            assert_eq!(d, *result, 
                "result for {}({:?}, {:?}) should be {} but {} was given instead!", name, l, r, *result, d);
        }
    }

    fn test_dist_against_naif<F>(f: F, name: &str)
    where F: Fn(&Vec<Dna>, &Vec<Dna>) -> u64
    {
        let gdata = env::var("GENOME_DATA").expect("GENOME_DATA environnement variable cannot be found!");
        let filenames = &[
            "Inst_0000010_44.adn",
            "Inst_0000010_7.adn",
            "Inst_0000010_8.adn",
            "Inst_0000012_13.adn",
            // "Inst_0000012_56.adn",
            // "Inst_0000012_32.adn",
            // "Inst_0000013_56.adn",
        ];

        let testcases = filenames
            .iter()
            .map(|p| gdata.clone() + "/" + p)
            .map(|p| read_to_string(p).expect("cannot read file!"))
            .map(|s| s.parse::<DnaBlock>().expect("cannot parse file: {}"));

        for DnaBlock(l, r) in testcases {
            let d = f(&l, &r);
            let d2 = super::dist_naif::<DnaMetricSpace>(&l, &r);
            assert_eq!(d, d2, 
                "result for {}({:?}, {:?}) should be {} but {} was given instead!", name, l, r, d2, d);
        }
    }



    #[test]
    fn dist_naif_dna(){
        test_dist_3(|l: &Vec<Dna>, r: &Vec<Dna>| -> u64 {
            super::dist_naif::<DnaMetricSpace>(l, r)
        }, "dist_naif");
    }

    #[test]
    fn dist_1_dna(){
        test_dist_against_naif(|l: &Vec<Dna>, r: &Vec<Dna>| -> u64 {
            super::dist_1::<DnaMetricSpace>(l, r)
        }, "dist_1");
    }
}