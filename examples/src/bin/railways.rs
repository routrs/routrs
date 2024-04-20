use std::time::{Duration, Instant};

use routrs::prelude::*;
use routrs_railways::RAILWAYS;

fn main() {
    let mut iter = 0;
    let mut total_time = Duration::new(0, 0);

    let first_node = RAILWAYS.get(1).unwrap();
    for node in RAILWAYS.iter_nodes() {
        iter += 1;
        let start = Instant::now(); // Start timing
        let (distance, path, path_type) = RAILWAYS.distance(first_node, node).unwrap();
        let duration = start.elapsed();
        total_time += duration;
        let avg_time = total_time / iter;

        println!(
            "{:?}\t{:?}\t{:?}\t\tdistance: {:?} km ({:?} nodes) {:?}",
            avg_time,
            first_node.geoloc(),
            node.geoloc(),
            distance,
            path.len(),
            path_type
        );
    }
}
