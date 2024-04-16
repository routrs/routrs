use routrs::Geolocalizable;

use routrs_railways::RAILWAYS;

fn main() {
    println!("Railways {:#?} nodes", RAILWAYS.len());

    let total_railway_distance: f64 = RAILWAYS
        .nodes()
        .map(|node| {
            let destination_id = *node.waypoints.first().unwrap_or(&node.id);
            let destination = RAILWAYS.get(destination_id).expect("error");
            node.haversine(destination)
        })
        .sum();

    println!(
        "avg_railway_distance {:#?} km",
        total_railway_distance / RAILWAYS.len() as f64
    );
}
