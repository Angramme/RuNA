use simple_error::{SimpleError, bail};
use std::error::Error;
use std::str::FromStr;
use std::fmt::Display;

use crate::math::*;

/// Dna element, an item in a dna sequence
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Dna {
    A, C, T, G, Gap
}

impl Display for Dna {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dna::Gap => write!(f, "-"),
            _ => write!(f, "{:?}", self)
        }
    }
}

impl std::str::FromStr for Dna {
    type Err = SimpleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "a" => Ok(Self::A),
            "C" | "c" => Ok(Self::C),
            "T" | "t" => Ok(Self::T),
            "G" | "g" => Ok(Self::G),
            "-" | "_" => Ok(Self::Gap),
            x => bail!(&format!("invalid string passed \"{}\"", x)[..]),
        }
    }
}

/// Metric space defined for dna sequences, it is defined as mentioned in the assignement
#[derive(Debug, PartialEq, Eq)]
pub struct DnaMetricSpace;
impl MetricSpace for DnaMetricSpace {
    type Cost = u64;
    type Item = Dna; 

    const DEL: Self::Cost = 2;
    const INS: Self::Cost = 2;
    const GAP: Self::Item = Dna::Gap;
    const ZEROCOST: Self::Cost = 0;
    const INFCOST: Self::Cost = Self::Cost::MAX;

    fn sub(a: Self::Item, b: Self::Item) -> Self::Cost { 
        use Dna as D;
        match (a, b) {
            (D::Gap, _) | (_, D::Gap) => panic!("invalid argument passed!"),
            (x, y) if x == y => 0,
            (D::A, D::T) | (D::G, D::C) | (D::T, D::A) | (D::C, D::G) => 3,
            (_, _) => 4
        } 
    }
}

/// Data structure representing a pair of Dna's
#[derive(Debug, PartialEq, Eq)]
pub struct DnaBlock(pub Vec<Dna>, pub Vec<Dna>);

impl FromStr for DnaBlock {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ls = s.lines();
        
        let n = ls.next()
            .ok_or("couldn't read line containing n!")?
            .parse::<usize>()?;
        let m = ls.next()
            .ok_or("couldn't read line containing m!")?
            .parse::<usize>()?;
        
        let mut xs = Vec::with_capacity(n);
        for s in ls.next()
            .ok_or("couldn't read line containing xs!")?
            .split_ascii_whitespace()
            { xs.push(s.parse::<Dna>()?); }
        
        let mut ys = Vec::with_capacity(m);
        for s in ls.next()
            .ok_or("couldn't read line containing ys!")?
            .split_ascii_whitespace()
            { ys.push(s.parse::<Dna>()?); }
        
        if xs.len() != n { bail!(format!("size mismatch between the number of DNA letters {} and the length provided! {}", xs.len(), n)); }
        if ys.len() != m { bail!(format!("size mismatch between the number of DNA letters {} and the length provided! {}", ys.len(), m)); }

        Ok(DnaBlock(xs, ys))
    }
}

#[cfg(test)]
mod tests{
    use crate::io::read_test_inst;

    use super::Dna::*;
    use super::DnaBlock;

    #[test]
    fn read_double_dna_block(){
        let x = read_test_inst("Inst_0000010_44.adn").expect("the reader cannot read the file!");
        assert_eq!(x, DnaBlock(vec![T, A, T, A, T, G, A ,G ,T, C], vec![T, A, T, T, T]), "the reader is not correct!")
    }
}

