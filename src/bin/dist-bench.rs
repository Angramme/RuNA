
use std::time::{Duration, Instant};
use seq_align::{math::*, dna::*};
use std::fs::read_to_string;

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
    let sizes1 = [10, 12, 13, 14, 20, 50, 100, 500].iter().copied();
    let sizes2 = (3..).map(|i| usize::pow(10, i));
    let sizes = sizes1.chain(sizes2); // this is infinite ♾, ~~waw, so cool ✨
    let secsizes = [7, 8, 13, 45];
    
    let filenames = sizes
        .map(|size| (size, secsizes
            .into_iter()
            .map(move |size2| format!("./tests/Instances_genome/Inst_{:07}_{}.adn", size, size2))
        ));

    let blocks = filenames
        .map(|(size, ps)| (size, ps
            .map(read_to_string)
            .find_map(|x| x.ok())
            .expect("cannot open file!")
        ))
        .map(|(size, str)| (size, str.parse::<DnaBlock>().expect("cannot parse file!")));

    let times = blocks
        .map(|(size, dna)| (size, lapse(|| f(dna))));

    times
        .map(|(size, time)| {
            println!("👍 completed call of size {} in {}s", size, time.as_secs());
            (size, time)
        })
        .take_while(|(_, time)| *time < Duration::from_secs(60))
        .last().map_or(0, |(size, _)| size)
}

fn main(){
    let dist_naif_limit = lapse_limit(|DnaBlock(l, r)| {
        dist_naif::<DnaMetricSpace, _>(l.iter().copied(), r.iter().copied());
    });

    println!("the limit of ✨dist_naif✨ is {}", dist_naif_limit); // this gives 12, 13 executes in 122s
}