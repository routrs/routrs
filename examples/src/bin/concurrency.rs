use rayon::prelude::*;
use std::time::Instant;

use routrs::prelude::*;
use routrs::MARITIME;

fn main() {
    let nodes: Vec<&Node> = MARITIME.nodes().take(100).collect();
    let others: Vec<&Node> = nodes.iter().rev().cloned().take(20).collect();

    let start = Instant::now(); // Start timing
    nodes.par_iter().for_each(|node| {
        let distance: f64 = others
            .iter()
            .map(|other| {
                MARITIME
                    .distance(*node, *other)
                    .map(|(distance, _, _)| distance)
                    .unwrap_or(0.0)
            })
            .sum();

        println!("{:?}\tdistance: {:?} km ", node.geoloc(), distance,);
    });
    println!("duration: {:?}", start.elapsed())
}
