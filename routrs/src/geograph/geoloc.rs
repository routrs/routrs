const EARTH_RADIUS_KM: f64 = 6_371.0;
const KM_PER_DEGREE_LAT: f64 = 111.0; // Roughly 111 km per degree of latitude

/// Calculate the distance in kilometers for one degree of longitude at a given latitude
fn km_per_degree_lng(lat: f64) -> f64 {
    (EARTH_RADIUS_KM * (lat.to_radians().cos() * 2.0 * std::f64::consts::PI / 360.0)).abs()
}

pub type Coord = f64;
pub type Lat = Coord;
pub type Lng = Coord;
/// Represents a geographic location with latitude and longitude coordinates.
pub type Geoloc = (Lat, Lng);

pub trait Geolocalizable {
    fn geoloc(&self) -> Geoloc;
    fn lat(&self) -> Lat {
        self.geoloc().0
    }
    fn lng(&self) -> Lng {
        self.geoloc().1
    }

    /// Calculate the Manhattan distance between two geographic coordinates
    fn manhattan(&self, destination: &impl Geolocalizable) -> f64 {
        let lat_distance = (destination.lat() - self.lat()).abs() * KM_PER_DEGREE_LAT;
        let mid_lat = (self.lat() + destination.lat()) / 2.0;
        let lng_distance = (destination.lng() - self.lng()).abs() * km_per_degree_lng(mid_lat);

        lat_distance + lng_distance
    }

    /// Calculate the Haversine distance between two geographic coordinates
    fn haversine(&self, destination: &impl Geolocalizable) -> f64 {
        let lat1 = self.lat().to_radians();
        let lng1 = self.lng().to_radians();
        let lat2 = destination.lat().to_radians();
        let lng2 = destination.lng().to_radians();
        let d_lat = lat2 - lat1;
        let d_lng = lng2 - lng1;
        let sin_half_d_lat = (d_lat / 2.0).sin();
        let sin_half_d_lng = (d_lng / 2.0).sin();

        let a = sin_half_d_lat * sin_half_d_lat
            + lat1.cos() * lat2.cos() * sin_half_d_lng * sin_half_d_lng;
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        EARTH_RADIUS_KM * c
    }
}

impl Geolocalizable for Geoloc {
    fn geoloc(&self) -> Geoloc {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct GeolocMock {
        geoloc: Geoloc,
    }

    impl Geolocalizable for GeolocMock {
        fn geoloc(&self) -> Geoloc {
            self.geoloc
        }
    }

    #[test]
    fn test_haversine_distance() {
        let a = GeolocMock { geoloc: (1.0, 2.0) };
        let b = GeolocMock { geoloc: (3.0, 4.0) };

        assert_eq!(a.haversine(&b), 314.4029510236249);
    }
}
