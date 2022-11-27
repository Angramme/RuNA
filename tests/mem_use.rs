use std::fs::read_to_string;
use std::env;
use runa::{dna::{DnaBlock, DnaMetricSpace as Dms}, math::*};

fn main(){
    if env::args().nth(1) != Some("mem_use".to_string()) {
        println!("an argument saying \"mem_use\" is required");
        return;
    }
    let gdata = env::var("GENOME_DATA").expect("GENOME_DATA environnement variable cannot be found!");
    // let fname = gdata.clone() + "/Inst_0010000_7.adn";
    let fname = gdata.clone() + "/Inst_0000013_45.adn";
    let s = read_to_string(fname).expect("cannot read file!");
    let block = s.parse::<DnaBlock>().expect("cannot parse file: {}");

    dist_naif::<Dms>(&block.0, &block.1);
}