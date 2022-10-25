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
pub fn dist_naif<M, I>(x: I, y: I) -> M::Cost
where M: MetricSpace, I: Iterator<Item = M::Item> + Clone
{
    dist_naif_rec::<M, _>(x, y, M::ZEROCOST, M::INFCOST)
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

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::dna::{DnaMetricSpace, DnaBlock};

    #[test]
    fn dist_naif_dna(){
        let filenames = &[
            (10, "./tests/Instances_genome/Inst_0000010_44.adn"),
            (8, "./tests/Instances_genome/Inst_0000010_7.adn"),
            (2, "./tests/Instances_genome/Inst_0000010_8.adn")
        ];

        let testcases = filenames
            .iter()
            .map(|(t, p)| (t, read_to_string(p).expect("cannot read file! {}")))
            .map(|(t, s)| (t, s.parse::<DnaBlock>().expect("cannot parse file: {}")));

        for (result, DnaBlock(l, r)) in testcases {
            let d = super::dist_naif::<DnaMetricSpace, _>(l.iter().copied(), r.iter().copied());
            assert_eq!(d, *result, 
                "result for dist_naif({:?}, {:?}) should be {} but {} was given instead!", l, r, *result, d);
        }
    }
}