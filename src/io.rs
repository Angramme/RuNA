//! Input/Output library of the crate.

use crate::dna::DnaBlock;
use std::error::Error;
use std::fs::read_to_string;
use std::env;


/// read a test instance of a given size 
pub fn read_test_inst_of_size(size: usize) -> Result<DnaBlock, Box<dyn Error>>
{
    let secsizes = [7, 8, 13, 45, 32, 56, 89, 76, 77, 3, 20, 6];
    let gdata = env::var("GENOME_DATA")?;
    secsizes
        .into_iter()
        .map(|size2| format!("{}/Inst_{:07}_{}.adn", gdata, size, size2))
        .map(read_to_string) // try opening the file
        .find_map(|x| x.ok()) // open first existing file
        .ok_or("couldn't read file")? 
        .parse::<DnaBlock>()
}

/// read test instance by filename
pub fn read_test_inst(filename: &str) -> Result<DnaBlock, Box<dyn Error>>
{
    let gdata = env::var("GENOME_DATA")?;
    let f = gdata + "/" + filename;
    let f = read_to_string(f)?;
    f.parse::<DnaBlock>()
}

/// read all test instances lazily, indeed it will load the instances in memory on demand
pub fn read_test_insts_all<'a>() -> impl Iterator<Item = (usize, DnaBlock)> + 'a
{
    let filenames = [        
        "Inst_0000010_44.adn",
        "Inst_0000010_7.adn",
        "Inst_0000010_8.adn",
        "Inst_0000012_13.adn",
        "Inst_0000012_32.adn",
        "Inst_0000012_56.adn",
        "Inst_0000013_45.adn",
        "Inst_0000013_56.adn",
        "Inst_0000013_89.adn",
        "Inst_0000014_23.adn",
        "Inst_0000014_7.adn",
        "Inst_0000014_83.adn",
        "Inst_0000015_2.adn",
        "Inst_0000015_4.adn",
        "Inst_0000015_76.adn",
        "Inst_0000020_17.adn",
        "Inst_0000020_32.adn",
        "Inst_0000020_8.adn",
        "Inst_0000050_3.adn",
        "Inst_0000050_77.adn",
        "Inst_0000050_9.adn",
        "Inst_0000100_3.adn",
        "Inst_0000100_44.adn",
        "Inst_0000100_7.adn",
        "Inst_0000500_3.adn",
        "Inst_0000500_8.adn",
        "Inst_0000500_88.adn",
        "Inst_0001000_2.adn",
        "Inst_0001000_23.adn",
        "Inst_0001000_7.adn",
        "Inst_0002000_3.adn",
        "Inst_0002000_44.adn",
        "Inst_0002000_8.adn",
        "Inst_0003000_1.adn",
        "Inst_0003000_10.adn",
        "Inst_0003000_25.adn",
        "Inst_0003000_45.adn",
        "Inst_0005000_32.adn",
        "Inst_0005000_33.adn",
        "Inst_0005000_4.adn",
        "Inst_0008000_32.adn",
        "Inst_0008000_54.adn",
        "Inst_0008000_98.adn",
        "Inst_0010000_50.adn",
        "Inst_0010000_7.adn",
        "Inst_0010000_8.adn",
        "Inst_0015000_20.adn",
        "Inst_0015000_3.adn",
        "Inst_0015000_30.adn",
        "Inst_0020000_5.adn",
        "Inst_0020000_64.adn",
        "Inst_0020000_77.adn",
        "Inst_0050000_6.adn",
        "Inst_0050000_63.adn",
        "Inst_0050000_88.adn",
        "Inst_0100000_11.adn",
        "Inst_0100000_3.adn",
        "Inst_0100000_76.adn",
        ];

    let sizes = filenames
        .into_iter()
        .map(|s| s
            .split('_')
            .nth(1)
            .expect("invalid filename!")
            .parse::<usize>()
            .expect("invalid filename!"));

    let blocks = filenames
        .into_iter()
        .map(read_test_inst)
        .map(|e| e.expect("cannot parse file!"));

    sizes.zip(blocks)
}

/// read test instances lazily, indeed it will load the instances in memory on demand. The sizes of the instances will be unique.
pub fn read_test_insts_by_size<'a>() -> impl Iterator<Item = (usize, DnaBlock)> + 'a
{
    let sizes = [10, 12, 13, 14, 20, 50, 100, 500, 1000, 2000, 3000, 5000, 8000, 10000, 15000, 20000, 50000, 100000].into_iter();

    let blocks = sizes.clone()
        .map(read_test_inst_of_size)
        .map(|str| str.expect("cannot parse file!"));

    sizes.zip(blocks)
}

#[cfg(test)]
mod tests{
    use crate::dna::Dna::*;
    use super::{DnaBlock, read_test_inst};

    #[test]
    fn read_double_dna_block(){
        let x = read_test_inst("Inst_0000010_44.adn");
        match x {
            Err(x) => panic!("bad read, error: {}", x),
            Ok(x) => 
                assert_eq!(x, DnaBlock(vec![T, A, T, A, T, G, A ,G ,T, C], vec![T, A, T, T, T]), "the reader is not correct!")
        }
    }
}