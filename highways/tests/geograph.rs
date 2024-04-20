use routrs::prelude::*;
use routrs_highways::HIGHWAYS;

#[test]
fn it_reads_highways() {
    assert_eq!(HIGHWAYS.iter_nodes().count(), 560282);
}

#[test]
fn it_calculates_highway_distance() {
    let from: Geoloc = (31.33068357, 120.902694); // Kunshan, Suzhou, Jiangsu, China
    let to: Geoloc = (31.05287995, 121.2232226); // Songjiang District, Shanghai, China
    let (distance, path, path_type) = HIGHWAYS.distance(&from, &to).unwrap();

    assert_eq!(distance, 57.237115955889074);
    assert_eq!(path.len(), 39);
    assert_eq!(path_type, PathType::ViaWaypoints);
}
