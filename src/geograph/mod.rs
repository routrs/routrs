pub mod geoloc;
pub mod json;

use std::collections::HashMap;

pub use geoloc::{Coord, Geoloc, Geolocalizable, Lat, Lng};

/// Represents a unique identifier for a node.
pub type NodeId = i32;

/// Represents a node in a geograph.
/// Implements the `Geolocalizable` trait.
/// It has an identifiier and geographic location, as well as
/// references to the connected nodes or waypoints.
#[derive(Debug)]
pub struct Node {
    pub id: NodeId,
    pub waypoints: Vec<NodeId>,
    geoloc: Geoloc,
}

impl Node {
    pub fn new<I: IntoIterator<Item = NodeId>>(id: NodeId, geoloc: Geoloc, waypoints: I) -> Self {
        Self {
            id,
            geoloc,
            waypoints: waypoints.into_iter().collect(),
        }
    }
}

impl Geolocalizable for Node {
    fn geoloc(&self) -> Geoloc {
        self.geoloc
    }
}

#[derive(Debug)]
pub struct Geograph {
    pub name: String,
    graph: HashMap<NodeId, Node>,
}

impl Geograph {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            graph: HashMap::new(),
        }
    }

    pub fn add(&mut self, node: Node) {
        self.graph.insert(node.id, node);
    }

    pub fn get(&self, id: NodeId) -> Option<&Node> {
        self.graph.get(&id)
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.graph.values()
    }

    pub fn len(&self) -> usize {
        self.graph.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
