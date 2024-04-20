const EARTH_RADIUS_KM: f64 = 6_371.0;

/// Represents a geographic coordinate.
pub type Coord = f64;
/// Represents a latitude coordinate.
pub type Lat = Coord;
/// Represents a longitude coordinate.
pub type Lng = Coord;

/// Represents a geographic location with latitude and longitude coordinates.
pub type Geoloc = (Lat, Lng);

/// A trait for types that can be geolocalized and have geographic coordinates.
/// The haversine distance can be calculated between two geolocalizable objects.
pub trait Geolocalizable {
    fn geoloc(&self) -> Geoloc;
    fn lat(&self) -> Lat {
        self.geoloc().0
    }
    fn lng(&self) -> Lng {
        self.geoloc().1
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Distance(pub f64);

impl Eq for Distance {}
impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Distance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

/// Represents a path between two geolocalizable objects.
/// The path is represented as a vector of geolocalizable objects.
/// It's length can be calculated as the sum of the haversine distances between
/// each pair of geolocalizable objects.
/// Also, paths can be accessed and modified like a Vec.
#[derive(Debug)]
pub struct Path<T: Geolocalizable>(Vec<T>);

// Allow accessing elements like a Vec
impl<T: Geolocalizable> std::ops::Deref for Path<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// Allow modifying elements like a Vec
impl<T: Geolocalizable> std::ops::DerefMut for Path<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T: Geolocalizable> From<Vec<T>> for Path<T> {
    fn from(v: Vec<T>) -> Self {
        Self(v)
    }
}

impl<T: Geolocalizable> Path<T> {
    pub fn length(&self) -> f64 {
        self.iter()
            .zip(self.iter().skip(1))
            .map(|(node, next)| node.haversine(next))
            .sum()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    mod geolocalizable {
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
}
