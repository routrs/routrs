use super::{geoloc::Geolocalizable, Geoloc};

/// Represents a unique identifier for a node.
pub type NodeId = i32;

/// Represents a node in a geograph.
/// Implements the `Geolocalizable` trait.
/// It has an identifiier and geographic location, as well as
/// references to the connected nodes or waypoints.
pub struct Node {
    pub id: NodeId,
    pub waypoints: Vec<NodeId>,
    geoloc: Geoloc,
}

impl Node {
    pub fn new(id: NodeId, geoloc: Geoloc, waypoints: Vec<NodeId>) -> Self {
        Self {
            id,
            geoloc,
            waypoints,
        }
    }
}

impl Geolocalizable for Node {
    fn geoloc(&self) -> Geoloc {
        self.geoloc
    }
}
