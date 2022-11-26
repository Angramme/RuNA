
use std::time::{Duration, Instant};
use chrono::prelude::Local;
use seq_align::{math::*, dna::*};
use seq_align::io::read_test_data;
use std::env;

fn lapse<F>(f: F) -> Duration 
where F: FnOnce()
{
    let start = Instant::now();
    f();
    start.elapsed()
}

fn lapse_sequence<'a, F>(name: &'a str, f: F) -> impl Iterator<Item=(Duration, usize)> + 'a
where F: Fn(DnaBlock) + 'a
{
    let blocks = read_test_data();
    
    blocks
        .map(move |(size, dna)| (lapse(|| f(dna)), size)) // measure execution time
        .inspect(move |(time, size)| println!("👍 [{}] completed call of size {} in {}s (time: {})", name, size, time.as_secs_f64(), Local::now().time()))
}

fn lapse_limit<F>(name: &str, f: F) -> (&str, usize) 
where F: Fn(DnaBlock)
{
    let lim = lapse_sequence(name, f)
    .take_while(|(time, _)| *time < Duration::from_secs(60))
    .last().map_or(0, |(_, size)| size);
    
    (name, lim)
}

fn all_limits(){
    let limits = vec![
        lapse_limit("dist_2", |DnaBlock(l, r)| {dist_2::<DnaMetricSpace>(&l, &r);}), // 100000 in 68.3044632s
        lapse_limit("dist_1", |DnaBlock(l, r)| {dist_1::<DnaMetricSpace>(&l, &r);}), // 10000 before memory limit.
        lapse_limit("dist_naif", |DnaBlock(l, r)| {dist_naif::<DnaMetricSpace>(&l, &r);}), // this gives 14, 15 executes in much more
    ];
    
    for (name, limit) in limits {
        println!("the limit of ✨{}✨ is {}", name, limit); 
    }
}

fn gnuplot(){
    let times = 
        lapse_sequence("dist_2", |DnaBlock(l, r)| {dist_2::<DnaMetricSpace>(&l, &r);})
        .take_while(|(time, _)| *time < Duration::from_secs_f64(1.0));
    
    for (time, size) in times {
        print!("({},{:.6})", size, time.as_secs_f64());
    }
}

fn main(){
    match env::args().nth(1) {
        Some(x) if x == "gnuplot" => gnuplot(),
        _ => all_limits(),
    }
}