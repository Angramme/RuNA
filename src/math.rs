#![allow(dead_code)]
// use std::str::Chars;

trait MetricSpace {
    type Item;
    type Cost: Ord + std::ops::Add<Output = Self::Cost>;

    const nocost: Self::Cost;
    const del: Self::Cost;
    const ins: Self::Cost;
    fn sub(a: Self::Item, b: Self::Item) -> Self::Cost;
}

struct StrMetricSpace;

impl MetricSpace for StrMetricSpace {
    type Item = char;
    type Cost = u64;

    const nocost: u64 = 0;
    const del: u64 = 1;
    const ins: u64 = 1;
    fn sub(a: char, b: char) -> u64 { if a == b {0} else {1} }
}

fn dist_naif<M, I>(x: I, y: I) -> M::Cost
where M: MetricSpace, I: Iterator<Item = M::Item>
{
    dist_naif_rec::<M, _>(x, y, M::nocost, None)
}

fn dist_naif_rec<T, I>(mut xi: I, mut yi: I, c: T::Cost, dist: Option<T::Cost>) -> T::Cost
where T: MetricSpace, I: Iterator<Item = T::Item>,
{
    match (xi.next(), yi.next()) {
        (None, None) => match dist { Some(dist) => c.min(dist), None => c },
        (Some(xj), Some(yj)) => dist_naif_rec::<T, I>(xi, yi, c + T::sub(xj, yj), dist),
        (Some(_), None) => dist_naif_rec::<T, I>(xi, yi, c + T::del, dist),
        (None, Some(_)) => dist_naif_rec::<T, I>(xi, yi, c + T::ins, dist),
    }
}

#[cfg(test)]
mod tests {
    use crate::math::StrMetricSpace;

    #[test]
    fn dist_naif() {
        let test_results = &[
            ("abc", "abc", 0),
            ("abc", "abs", 1),
        ];

        for (x ,y, result) in test_results {
            assert_eq!(super::dist_naif::<StrMetricSpace, _>(x.chars(), y.chars()), *result);
        }
    }
}