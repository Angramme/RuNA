
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

fn lapse_limit<F>(name: &str, f: F) -> (&str, usize) 
where F: Fn(DnaBlock)
{
    let sizes = {
        let sizes1 = [10, 12, 13, 14, 20, 50, 100, 500].into_iter();
        let sizes2 = (3..).map(|i| usize::pow(10, i));
        sizes1.chain(sizes2) // this is infinite ♾, ~~waw, so cool ✨
    };
    let secsizes = [7, 8, 13, 45, 32, 56, 89, 76, 77]; // this is here because the endings of files differ for different sizes
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
    
    let lim = blocks
        .map(|dna| lapse(|| f(dna))) // measure execution time
        .zip(sizes)
        .inspect(|(time, size)| println!("👍 [{}] completed call of size {} in {}s", name, size, time.as_secs_f64()))
        .take_while(|(time, _)| *time < Duration::from_secs(60))
        .last().map_or(0, |(_, size)| size);
    
    (name, lim)
}

fn main(){
    let limits = vec![
        lapse_limit("dist_2", |DnaBlock(l, r)| {dist_2::<DnaMetricSpace>(&l, &r);}), // 100000 in 68.3044632s
        lapse_limit("dist_1", |DnaBlock(l, r)| {dist_1::<DnaMetricSpace>(&l, &r);}), // 10000 before memory limit.
        lapse_limit("dist_naif", |DnaBlock(l, r)| {dist_naif::<DnaMetricSpace>(&l, &r);}), // this gives 14, 15 executes in much more
    ];

    for (name, limit) in limits {
        println!("the limit of ✨{}✨ is {}", name, limit); 
    }
}