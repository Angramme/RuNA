//! A binary for memory usage tests

use std::env;
use runa::{dna::{DnaMetricSpace as Dms}, math::*, io::read_test_inst};


fn main_dist_naif(){
    let block = read_test_inst("Inst_0000013_45.adn").expect("cannot read block!");
    let res = dist_naif::<Dms>(&block.0, &block.1);
    println!("{}", res);
}

fn main_dist_1(){
    let block = read_test_inst("Inst_0010000_7.adn").expect("cannot read block!");
    let res = dist_1::<Dms>(&block.0, &block.1);
    println!("{}", res);
}

fn main_sol_1(){
    let block = read_test_inst("Inst_0010000_7.adn").expect("cannot read block!");
    let res = sol_1::<Dms>(&block.0, &block.1);
    println!("{}", res);
}

fn main_prog_dyn(){
    let block = read_test_inst("Inst_0010000_7.adn").expect("cannot read block!");
    let res = prog_dyn::<Dms>(&block.0, &block.1);
    println!("{}", res.0);
}

fn main_dist_2(){
    let block = read_test_inst("Inst_0010000_7.adn").expect("cannot read block!");
    dist_2::<Dms>(&block.0, &block.1);
}

fn main_sol_2(){
    // TODO
    // let block = read_test_inst("Inst_0000013_45.adn").expect("cannot read block!");
    // dist_naif::<Dms>(&block.0, &block.1);
}


fn main(){
    let funcs = [
        ("dist_naif", main_dist_naif as fn()),
        ("dist_1", main_dist_1),
        ("dist_2", main_dist_2),
        ("prog_dyn", main_prog_dyn),
        ("sol_1", main_sol_1),
        ("sol_2", main_sol_2),
    ];
    let arg = env::args().nth(1).expect("an argument is required!"); 

    let f = funcs.into_iter().find(|&(n, _)| n == arg).map(|(_, b)| b);
    
    match f {
        None => {
            println!("Invalid argument! Function {} is not supported by this program", arg);
            println!("please select one function among: ");
            for (n, _) in funcs {
                println!("  - {}", n);
            }
        },
        Some(f) => f(),
    }
}