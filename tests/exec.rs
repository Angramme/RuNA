use runa::math::{dist_1, dist_2, dist_naif, sol_1, sol_2, prog_dyn};
use runa::dna::{DnaMetricSpace as Dms, DnaBlock};
use std::{env, process};
use std::fs::read_to_string;
use std::{iter, error::Error};



fn run() -> Result<(), Box<dyn Error>>
{
    let dists = [
        ("dist_1", dist_1::<Dms> as fn(_, _) -> _),
        ("dist_2", dist_2::<Dms>),
        ("dist_naif", dist_naif::<Dms>),
    ];
    let sols = [
        ("sol_1", sol_1::<Dms> as fn(_, _) -> _),
        ("sol_2", sol_2::<Dms>),
    ];

    let block = {
        let arg = env::args().nth(1).expect("instance location is required!");
        read_to_string(arg)?.parse::<DnaBlock>()?
    };

    let block = &block;

    let funcs = 
        dists
        .into_iter()
        .map(|(n, f)| (n, Box::new(move || {
            let res = f(block.0.as_slice(), block.1.as_slice());
            println!("distance: {}", res);
        }) as Box<dyn Fn()>))
        .chain(
            sols
            .iter()
            .map(|&(n, f)| (n, Box::new(move || {
                let res = f(block.0.as_slice(), block.1.as_slice());
                println!("alignement: \n{}", res);
            }) as Box<dyn Fn()>))
        )
        .chain(
            iter::once(("prog_dyn", prog_dyn::<Dms>))
            .map(|(n, f)| (n, Box::new(move || {
                let res = f(block.0.as_slice(), block.1.as_slice());
                println!("alignement: \n{}", res.1);
                println!("coût: {}", res.0);
            }) as Box<dyn Fn()>))
        );
    
    let func_arg = env::args().nth(2).expect("function parameter is required!");
    let func = funcs.clone().find(|&(n, _)| n == func_arg);

    match func {
        None => {
            println!("Invalid argument! Function {} is not supported by this program", func_arg);
            println!("please select one function among: ");
            for (n, _) in funcs {
                println!("  - {}", n);
            }
        },
        Some((_, func)) => func(),
    }

    Ok(())
}

fn main(){
    if let Err(e) = run() {
        println!("Application error: {:?}", e);
        process::exit(1);
    }
}
