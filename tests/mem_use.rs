use std::env;
use runa::{dna::{DnaMetricSpace as Dms}, math::*, io::read_test_inst};

fn main(){
    if env::args().nth(1) != Some("mem_use".to_string()) {
        println!("an argument saying \"mem_use\" is required");
        return;
    }
    let block = read_test_inst("Inst_0000013_45.adn").expect("cannot read block!");

    dist_naif::<Dms>(&block.0, &block.1);
}