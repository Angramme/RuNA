use simple_error::{SimpleError, bail};
use std::error::Error;
use std::{path::Path, fs::File};
use std::io::{BufReader, BufRead};

use crate::math::*;

#[derive(PartialEq, Debug)]
pub enum Dna {
    A, C, T, G, Gap
}

impl std::str::FromStr for Dna {
    type Err = SimpleError;
    // type Err = ();
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

pub struct DnaMetricSpace;
impl MetricSpace for DnaMetricSpace {
    type Cost = u64;
    type Item = Dna; 

    const DEL: Self::Cost = 1;
    const INS: Self::Cost = 1;
    const NOCOST: Self::Cost = 0;

    fn sub(a: Self::Item, b: Self::Item) -> Self::Cost { if a==b { 1 } else { 0 } }
}

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
            Some(read_double_dna_block(buf.chars()))
        }
    }
}

fn read_double_dna_block(mut ite: std::str::Chars) -> Result<(Vec<Dna>, Vec<Dna>), Box<dyn Error>>
{
    let read_line = |ite: &mut std::str::Chars| {
        ite
        .skip_while(|x| { x.is_whitespace() })
        .take_while(|&x| { x != '\n' })
        .collect::<String>()
    };

    let n = read_line(ite.by_ref()).parse::<usize>()?;
    let m = read_line(ite.by_ref()).parse::<usize>()?;
    let mut xs = Vec::new();
    for s in 
        read_line(ite.by_ref())
        .split_ascii_whitespace() {
            xs.push(s.parse::<Dna>()?);
        }
    let mut ys = Vec::new();
    for s in 
        read_line(ite.by_ref())
        .split_ascii_whitespace() {
            ys.push(s.parse::<Dna>()?);
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

