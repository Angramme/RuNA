use simple_error::{SimpleError, bail};
use std::error::Error;
use std::{path::Path, fs::File};
use std::io::{BufReader, BufRead};
use std::fmt::Display;

use crate::math::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Dna {
    A, C, T, G, Gap
}

impl Display for Dna {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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

/// Metric space for Dna
pub struct DnaMetricSpace;
impl MetricSpace for DnaMetricSpace {
    type Cost = u64;
    type Item = Dna; 

    const DEL: Self::Cost = 2;
    const INS: Self::Cost = 2;
    const ZEROCOST: Self::Cost = 0;
    const INFCOST: Self::Cost = Self::Cost::MAX;

    fn sub(a: Self::Item, b: Self::Item) -> Self::Cost { 
        use Dna as D;
        match (a, b) {
            (x, y) if x == y => 0,
            (D::A, D::T) | (D::G, D::C) | (D::T, D::A) | (D::C, D::G) => 3,
            (_, _) => 4
        } 
    }
}

/// An iterator type that reads a file lazily and gives dna blocks contained inside.
pub struct DnaBlocks{
    reader: Box<dyn BufRead>,
}

impl DnaBlocks {
    pub fn new(rd: Box<dyn BufRead>) -> Self {
        DnaBlocks{reader: rd}
    }
}
impl DnaBlocks {
    pub fn from_file(f: File) -> Self {
        let reader = Box::from(BufReader::new(f));
        Self::new(reader)
    }
    pub fn from_path<P>(path: P) -> std::io::Result<Self>
    where P: AsRef<Path>, 
    {
        let f = File::open(path)?;
        Ok(Self::from_file(f))
    }
}

impl Iterator for DnaBlocks {
    type Item = Result<(Vec<Dna>, Vec<Dna>), Box<dyn Error>>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = String::new();
        for _ in 0..4 { 
            if let Err(er) = self.reader.read_line(&mut buf) { 
                return Some(Err(Box::new(er))) 
            } 
        }
        if buf.is_empty() {
            None
        } else {
            Some(read_double_dna_block(buf.as_str()))
        }
    }
}

fn read_double_dna_block(ite: &str) -> Result<(Vec<Dna>, Vec<Dna>), Box<dyn Error>>
{ // TODO: replace with cleaner code
    let mut ls = ite.lines();

    let n = (if let Some(t) = ls.next() { t.parse::<usize>() } else { bail!("couldn't read line containing n!") })?;
    let m = (if let Some(t) = ls.next() { t.parse::<usize>() } else { bail!("couldn't read line containing m!") })?;
    
    let mut xs = Vec::with_capacity(n);
    match ls.next() {
        Some(t) => for s in t.split_ascii_whitespace(){
            xs.push(s.parse::<Dna>()?);
        },
        None => bail!("couldn't read line containing xs!"),
    }

    let mut ys = Vec::with_capacity(m);
    match ls.next() {
        Some(t) => for s in t.split_ascii_whitespace(){
            ys.push(s.parse::<Dna>()?);
        },
        None => bail!("couldn't read line containing ys!"),
    }
    
    if xs.len() != n { bail!(format!("size mismatch between the number of DNA letters {} and the length provided! {}", xs.len(), n)); }
    if ys.len() != m { bail!(format!("size mismatch between the number of DNA letters {} and the length provided! {}", ys.len(), m)); }

    Ok((xs, ys))    
}

#[cfg(test)]
mod tests{
    use super::Dna::*;
    use super::Path;

    use super::DnaBlocks;

    #[test]
    fn read_double_dna_block(){
        let path = Path::new("./tests/Instances_genome/Inst_0000010_44.adn");
        let mut bl = DnaBlocks::from_path(path).expect("cannot read file");
        let x = bl.next().expect("bad reading: file is not empty!");
        match x {
            Err(x) => panic!("bad read, error: {}", x),
            Ok(x) => 
                assert_eq!(x, (vec![T, A, T, A, T, G, A ,G ,T, C], vec![T, A, T, T, T]), "the reader is not correct!")
        }
    }
}

