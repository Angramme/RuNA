
use crate::math::*;

#[derive(PartialEq)]
enum Dna {
    A, C, T, G, Gap
}

struct DnaMetricSpace;
impl MetricSpace for DnaMetricSpace {
    type Cost = u64;
    type Item = Dna; 

    const DEL: Self::Cost = 1;
    const INS: Self::Cost = 1;
    const NOCOST: Self::Cost = 0;

    fn sub(a: Self::Item, b: Self::Item) -> Self::Cost { if a==b { 1 } else { 0 } }
}


