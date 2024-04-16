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

    pub fn add(&mut self, node: Node) -> &mut Self {
        self.graph.insert(node.id, node);
        self
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

#[cfg(test)]
mod tests {
    use super::*;

    fn geograph_fixture() -> Geograph {
        let node1 = Node::new(1, (0.0, 0.0), vec![2, 3]);
        let node2 = Node::new(2, (1.0, 1.0), vec![1]);
        let node3 = Node::new(3, (2.0, 2.0), vec![1]);

        let mut geograph = Geograph::new("Test Geograph");
        geograph.add(node1).add(node2).add(node3);
        geograph
    }

    #[test]
    fn test_add() {
        let geograph = geograph_fixture();

        assert_eq!(geograph.len(), 3);
        assert!(!geograph.is_empty());
    }

    #[test]
    fn test_get_existing_node() {
        let geograph = geograph_fixture();
        let retrieved_node = geograph.get(2);

        assert!(retrieved_node.is_some());
        assert_eq!(retrieved_node.unwrap().id, 2);
    }

    #[test]
    fn test_get_non_existent_node() {
        let geograph = geograph_fixture();
        let non_existent_node = geograph.get(4);

        assert!(non_existent_node.is_none());
    }

    #[test]
    fn test_nodes() {
        let geograph = geograph_fixture();
        let nodes: Vec<&Node> = geograph.nodes().collect();

        assert_eq!(nodes.len(), 3);
    }
}
