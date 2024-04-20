use routrs::concurrency::*;
use routrs::highways::GEOGRAPH as highways;
use routrs::prelude::*;

use std::env;
use std::process;
use std::time::Instant;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let num_nodes: usize = match args.get(1).unwrap_or(&"10".to_string()).parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Warning: 'num_nodes' must be an integer");
            process::exit(1);
        }
    };

    let legs: Vec<Leg<Geoloc>> = highways
        .iter_nodes()
        .take(num_nodes)
        .flat_map(|node| {
            highways
                .iter_nodes()
                .take(num_nodes)
                .map(|other| Leg((node.geoloc(), other.geoloc())))
        })
        .collect();

    let start = Instant::now();
    println!("calculating {:?} distances", legs.len());

    let results = highways.par_distance(&legs);

    let duration = start.elapsed();
    println!("total: {:?}", duration);
    println!("avg: {:?}", duration.div_f64(results.len() as f64))
}
