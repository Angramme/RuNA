
use std::time::{Duration, Instant};
use seq_align::{math::*, dna::*};
use std::fs::read_to_string;
use std::env;

fn lapse<F>(f: F) -> Duration 
where F: FnOnce()
{
    let start = Instant::now();
    f();
    start.elapsed()
}

fn lapse_limit<F>(f: F) -> usize 
where F: Fn(DnaBlock)
{
    let sizes = {
        let sizes1 = [10, 12, 13, 14, 20, 50, 100, 500].into_iter();
        let sizes2 = (3..).map(|i| usize::pow(10, i));
        sizes1.chain(sizes2) // this is infinite ♾, ~~waw, so cool ✨
    };
    let secsizes = [7, 8, 13, 45, 32, 56, 89, 76]; // this is here because the endings of files differ for different sizes
    let gdata = env::var("GENOME_DATA").expect("GENOME_DATA environnement variable cannot be found!");

    let blocks = sizes.clone()
        .map(|size| secsizes
            .into_iter()
            .map(|size2| format!("{}/Inst_{:07}_{}.adn", gdata, size, size2))
            .map(read_to_string) // try opening the file
            .find_map(|x| x.ok()) // open first existing file
            .expect("cannot open file!")
        )
        .map(|str| str.parse::<DnaBlock>().expect("cannot parse file!"));
    
    blocks
        .map(|dna| lapse(|| f(dna))) // measure execution time
        .zip(sizes)
        .inspect(|(time, size)| println!("👍 completed call of size {} in {}s", size, time.as_secs_f64()))
        .take_while(|(time, _)| *time < Duration::from_secs(60))
        .last().map_or(0, |(_, size)| size)
}

fn main(){
    let dist_naif_limit = lapse_limit(|DnaBlock(l, r)| {
        dist_naif::<DnaMetricSpace, _>(l.into_iter(), r.into_iter());
    });

    println!("the limit of ✨dist_naif✨ is {}", dist_naif_limit); // this gives 12, 13 executes in 122s
}