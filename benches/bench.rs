
use std::time::{Duration, Instant};
use chrono::prelude::Local;
use runa::{math::*, dna::*, dna::DnaMetricSpace as Dms};
use runa::io::read_test_insts_by_size;
use std::env;

type DistFunc = fn(&[Dna], &[Dna]) -> <Dms as MetricSpace>::Cost;
type SolFunc = fn(&[Dna], &[Dna]) -> Align<Dms>;
const DIST_FUNCTIONS: [(&str, DistFunc); 3] = [
    ("dist_1", dist_1::<Dms>),
    ("dist_2", dist_2::<Dms>),
    ("dist_naif", dist_naif::<Dms>),
    ];
const SOL_FUNCTIONS: [(&str, SolFunc); 2] = [
    ("sol_1", sol_1::<Dms>),
    ("sol_2", sol_2::<Dms>),
    ];

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
    let blocks = read_test_insts_by_size();
    
    blocks
        .map(move |(size, dna)| (lapse(|| f(dna)), size)) // measure execution time
        .inspect(move |(time, size)| println!("👍 [{}] completed call of size {} in {}s (time: {})", 
            name, size, time.as_secs_f64(), Local::now().time()))
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
    let dists = DIST_FUNCTIONS
        .iter().copied()
        .map(|(s, f)| (s, Box::new(move |DnaBlock(l, r)| { f(&l, &r); }) as Box<dyn Fn(DnaBlock)>));
    let sols = SOL_FUNCTIONS
        .iter().copied()
        .map(|(s, f)| (s, Box::new(move |DnaBlock(l, r)| { f(&l, &r); }) as Box<dyn Fn(DnaBlock)>));
    let mut funcs = dists.chain(sols);


    let fct = env::args().nth(2).expect("expected function as parameter!");
    let (_, fctptr) = 
        funcs
        .find(|&(f, _)| f == fct)
        .expect("function not found!");

    let times = 
        lapse_sequence(fct.as_str(), fctptr)
        .take_while(|(time, _)| *time < Duration::from_secs(60 * 10));

    let mut plot = vec![];
    
    for (time, size) in times {
        plot.push((time, size));
        println!("our plot so far looks like: \n===");
        for (time, size) in plot.iter() {
            print!("({},{:.6})", size, time.as_secs_f64());
        }
        println!("\n===")
    }
}

fn main(){
    match env::args().nth(1) {
        Some(x) if x == "gnuplot" => gnuplot(),
        _ => all_limits(),
    }
}