pub mod geograph;
pub use geograph::{Coord, Geograph, Geoloc, Geolocalizable, Lat, Lng, Node, NodeId};

use lazy_static::lazy_static;

use geograph::json::JsonGeograph;

lazy_static! {
    pub static ref MARNET: geograph::Geograph =
        serde_json::from_str::<JsonGeograph>(include_str!("../data/marnet.json"))
            .expect("Failed to parse Marnet Geograph JSON")
            .into();
    pub static ref HIGHWAYS: geograph::Geograph =
        serde_json::from_str::<JsonGeograph>(include_str!("../data/highways.json"))
            .expect("Failed to parse Highways Geograph JSON")
            .into();
    pub static ref RAILWAYS: geograph::Geograph =
        serde_json::from_str::<JsonGeograph>(include_str!("../data/railways.json"))
            .expect("Failed to parse Railways Geograph JSON")
            .into();
}
