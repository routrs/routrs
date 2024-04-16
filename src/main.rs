use routrs::{Geolocalizable, HIGHWAYS};

fn main() {
    println!("Marnet: {:#?} nodes", routrs::MARNET.len());
    println!("Highways {:#?} nodes", routrs::HIGHWAYS.len());
    println!("Railways {:#?} nodes", routrs::RAILWAYS.len());

    let total_highway_distance: f64 = routrs::HIGHWAYS
        .nodes()
        .map(|node| {
            let destination_id = *node.waypoints.first().unwrap_or(&node.id);
            let destination = HIGHWAYS.get(destination_id).expect("error");
            node.haversine(destination)
        })
        .sum();

    println!(
        "avg_highway_distance {:#?} km",
        total_highway_distance / routrs::HIGHWAYS.len() as f64
    );
}
