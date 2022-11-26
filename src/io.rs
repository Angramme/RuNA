use std::{path::Path, fs::File};
use std::io::{BufReader, BufRead};
use crate::dna::DnaBlock;
use std::error::Error;
use std::fs::read_to_string;
use std::env;

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

pub fn read_test_data<'a>() -> impl Iterator<Item = (usize, DnaBlock)> + 'a
{
    // let sizes = {
    //     let sizes1 = [10, 12, 13, 14, 20, 50, 100, 500].into_iter();
    //     let sizes2 = (3..=5).map(|i| usize::pow(10, i));
    //     sizes1.chain(sizes2)
    // };
    let sizes = [10, 12, 13, 14, 20, 50, 100, 500, 1000, 2000, 3000, 5000, 8000, 10000, 15000, 20000, 50000, 100000].into_iter();
    let secsizes = [7, 8, 13, 45, 32, 56, 89, 76, 77, 3, 20, 6]; // this is here because the endings of files differ for different sizes
    let gdata = env::var("GENOME_DATA").expect("GENOME_DATA environnement variable cannot be found!");

    let blocks = sizes.clone()
        .map(move |size| secsizes
            .into_iter()
            .map(|size2| format!("{}/Inst_{:07}_{}.adn", gdata, size, size2))
            .map(read_to_string) // try opening the file
            .find_map(|x| x.ok()) // open first existing file
            .expect("cannot open file!")
        )
        .map(|str| str.parse::<DnaBlock>().expect("cannot parse file!"));

    sizes.zip(blocks)
}

#[cfg(test)]
mod tests{
    use crate::dna::Dna::*;
    use super::DnaBlock;
    use std::env;

    use super::DnaBlocks;

    #[test]
    fn read_double_dna_block(){
        let path = env::var("GENOME_DATA")
            .expect("GENOME_DATA environnement variable cannot be found!") 
            + "/Inst_0000010_44.adn";

        let mut bl = DnaBlocks::from_path(path).expect("cannot read file");
        let x = bl.next().expect("bad reading: file is not empty!");
        match x {
            Err(x) => panic!("bad read, error: {}", x),
            Ok(x) => 
                assert_eq!(x, DnaBlock(vec![T, A, T, A, T, G, A ,G ,T, C], vec![T, A, T, T, T]), "the reader is not correct!")
        }
    }
}