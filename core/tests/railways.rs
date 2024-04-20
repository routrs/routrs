use routrs::prelude::*;
use routrs::railways;

#[test]
fn it_reads_railways_geograph() {
    assert_eq!(railways::GEOGRAPH.iter_nodes().count(), 274974);
}

#[test]
fn it_calculates_railway_distance() {
    let from: Geoloc = (48.8768, 2.3592); // Gare de l'Est, Paris, France
    let to: Geoloc = (43.3032, 5.3842); // Gare de Marseille-Saint-Charles, Marseille, France
    let (distance, path, path_type) = railways::GEOGRAPH.distance(&from, &to).unwrap();

    assert_eq!(distance, 749.4744344461568);
    assert_eq!(path.len(), 603);
    assert_eq!(path_type, PathType::ViaWaypoints);
}
