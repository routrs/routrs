use routrs::Geolocalizable;

use routrs::MARITIME;

fn main() {
    println!("Maritime {:#?} nodes", MARITIME.len());

    let total_maritime_distance: f64 = MARITIME
        .nodes()
        .map(|node| {
            let destination_id = *node.waypoints.first().unwrap_or(&node.id);
            let destination = MARITIME.get(destination_id).expect("error");
            node.haversine(destination)
        })
        .sum();

    println!(
        "avg_maritime_distance {:#?} km",
        total_maritime_distance / MARITIME.len() as f64
    );
}
