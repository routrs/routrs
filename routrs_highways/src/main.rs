use routrs::Geolocalizable;

use routrs_highways::HIGHWAYS;

fn main() {
    println!("Highways {:#?} nodes", HIGHWAYS.len());

    let total_highway_distance: f64 = HIGHWAYS
        .nodes()
        .map(|node| {
            let destination_id = *node.waypoints.first().unwrap_or(&node.id);
            let destination = HIGHWAYS.get(destination_id).expect("error");
            node.haversine(destination)
        })
        .sum();

    println!(
        "avg_highway_distance {:#?} km",
        total_highway_distance / HIGHWAYS.len() as f64
    );
}
