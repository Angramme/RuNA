//! The Math crate of the project

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
where M: MetricSpace, I: Iterator<Item = M::Item>
{
    dist_naif_rec::<M, _>(x, y, M::ZEROCOST, M::INFCOST)
}

pub fn dist_naif_rec<T, I>(mut xi: I, mut yi: I, c: T::Cost, mut dist: T::Cost) -> T::Cost
where T: MetricSpace, I: Iterator<Item = T::Item>
{
    let m = (xi.next(), yi.next());
    if let (None, None) = m { return c.min(dist); }
    if let (Some(xj), Some(yj)) = m { dist = dist_naif_rec::<T, I>(xi, yi, c + T::sub(xj, yj), dist); }
    if let (Some(_), None) = m { dist = dist_naif_rec::<T, I>(xi, yi, c + T::DEL, dist); }
    if let (None, Some(_)) = m { dist = dist_naif_rec::<T, I>(xi, yi, c + T::INS, dist); }
    dist
}

#[cfg(test)]
mod tests {
    use crate::dna::{DnaBlocks, DnaMetricSpace};

    struct StrMetricSpace;
    impl crate::math::MetricSpace for StrMetricSpace {
        type Item = char;
        type Cost = u64;

        const ZEROCOST: u64 = 0;
        const INFCOST: u64 = u64::MAX;
        const DEL: u64 = 1;
        const INS: u64 = 1;
        fn sub(a: char, b: char) -> u64 { if a == b {0} else {1} }
    }

    #[test]
    fn dist_naif() {
        let test_results = &[
            ("abc", "abc", 0),
            ("abc", "abs", 1),
        ];

        for (x ,y, result) in test_results {
            let v = super::dist_naif::<StrMetricSpace, _>(x.chars(), y.chars());
            assert_eq!(v, *result, "dist_naif::<StrMetricSpace, _>(\"{}\".chars(), \"{}\".chars()) = {} is not equal to the expected {}!", 
                x, y, v, *result);
        }
    }

    #[test]
    fn dist_naif_dna(){
        let block1 = DnaBlocks::from_path("./tests/Instances_genome/Inst_0000010_44.adn").expect("cannot read file");
        let block2 = DnaBlocks::from_path("./tests/Instances_genome/Inst_0000010_7.adn").expect("cannot read file");
        let block3 = DnaBlocks::from_path("./tests/Instances_genome/Inst_0000010_8.adn").expect("cannot read file");
        let blocks = block1.chain(block2).chain(block3);

        let testcases = blocks.zip([10, 8, 2].iter());

        for (inp, result) in testcases {
            let (l, r) = inp.expect("error reading input!");
            let d = super::dist_naif::<DnaMetricSpace, _>(l.iter().copied(), r.iter().copied());
            assert_eq!(d, *result, "result for dist_naif({:?}, {:?}) should be {} but {} was given instead!", l, r, *result, d);
        }
    }
}