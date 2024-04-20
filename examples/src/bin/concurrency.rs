use std::time::Instant;

use routrs::concurrency::*;
use routrs::prelude::*;
use routrs_maritime::MARITIME;

fn main() {
    let legs: Vec<Leg<Geoloc>> = MARITIME
        .iter_nodes()
        .take(100)
        .flat_map(|node| {
            MARITIME
                .iter_nodes()
                .take(20)
                .map(|other| Leg((node.geoloc(), other.geoloc())))
        })
        .collect();

    let start = Instant::now();
    println!("start: {:?} distances", legs.len());

    let results = MARITIME.par_distance(&legs);

    let duration = start.elapsed();
    println!("total: {:?}", duration);
    println!("avg: {:?}", duration.div_f64(results.len() as f64))
}
