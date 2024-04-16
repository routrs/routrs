/// Alias for a coordinate value.
pub type Coord = f64;
/// Alias for a latitude value.
pub type Lat = Coord;
/// Alias for a longitude value.
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
}
