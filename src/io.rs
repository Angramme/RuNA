use std::{path::Path, fs::File};
use std::io::{BufReader, BufRead};
use crate::dna::DnaBlock;
use std::error::Error;

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
    type Item = Result<DnaBlock, Box<dyn Error>>;
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
            Some(buf.as_str().parse::<DnaBlock>())
        }
    }
}

#[cfg(test)]
mod tests{
    use crate::dna::Dna::*;
    use super::DnaBlock;
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
                assert_eq!(x, DnaBlock(vec![T, A, T, A, T, G, A ,G ,T, C], vec![T, A, T, T, T]), "the reader is not correct!")
        }
    }
}