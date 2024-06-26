pub mod geoloc;

use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::iter;
use std::sync::Arc;

pub use geoloc::{Coord, Distance, Geoloc, Geolocalizable, Lat, Lng, Path};

#[derive(Debug, PartialEq)]
pub enum PathType {
    Direct,
    ViaWaypoints,
}

/// Represents a unique identifier for a node.
pub type NodeId = i32;

/// Represents a node in a geograph.
/// Implements the `Geolocalizable` trait.
/// It has an identifiier and geographic location, as well as
/// references to the connected nodes or waypoints.
#[derive(Debug, Clone)]
pub struct Node {
    pub id: NodeId,
    pub waypoints: Arc<[NodeId]>,
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

pub type ShortestPath = (f64, Path<Geoloc>, PathType);

impl Geograph {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            graph: HashMap::new(),
        }
    }

    /// Finds the closest node in the geograph to the given location.
    /// Used to find the entry and exit points for the shortest path calculation.
    fn closest(&self, loc: &impl Geolocalizable) -> Option<&Node> {
        self.iter_nodes().min_by(|a, b| {
            a.haversine(loc)
                .partial_cmp(&b.haversine(loc))
                .unwrap_or(Ordering::Equal)
        })
    }

    /// Calculates the distance between two geolocations within the geograph
    /// using Dijsktra's algorithm and the Haversine formula.
    ///
    /// It first finds the closest nodes to the origin and destination, then
    /// runs Dijkstra's algorithm on the graph to find the shortest path
    /// between the two closest nodes.
    ///
    /// In case the destination is not reachable from the origin, it will
    /// calculate the direct Haversine distance betweeen the locations.
    ///
    /// It returns a tuple with:
    /// - Total distance traveled
    /// - List of geolocations along the path
    /// - PathType indicating if it was a direct path or went through nodes
    ///
    pub fn shortest_path(
        &self,
        origin: &impl Geolocalizable,
        destination: &impl Geolocalizable,
    ) -> ShortestPath {
        let direct_path =
            |o: Geoloc, d| (o.haversine(&d), Path::from(vec![o, d]), PathType::Direct);

        match (self.closest(origin), self.closest(destination)) {
            (Some(origin_closest), Some(destination_closest)) => {
                let shortest_path = self.dijsktra(origin_closest.id, destination_closest.id);

                match shortest_path {
                    Some(path) => {
                        let path: Path<Geoloc> = iter::once(origin.geoloc())
                            .chain(
                                path.iter()
                                    .filter_map(|id| self.get(*id).map(|node| node.geoloc())),
                            )
                            .chain(iter::once(destination.geoloc()))
                            .collect::<Vec<_>>()
                            .into();
                        (path.length(), path, PathType::ViaWaypoints)
                    }
                    // If no path found, calculate the direct path between the origin and destination.
                    // This can happen if the destination is not reachable from the origin.
                    None => direct_path(origin.geoloc(), destination.geoloc()),
                }
            }
            // If any of the closest nodes is not found, calculate the direct path
            // between the origin and destination. Case for empty geographs.
            _ => direct_path(origin.geoloc(), destination.geoloc()),
        }
    }

    pub fn add(&mut self, node: Node) -> &mut Self {
        self.graph.insert(node.id, node);
        self
    }

    pub fn get(&self, id: NodeId) -> Option<&Node> {
        self.graph.get(&id)
    }

    pub fn get_copy(&self, id: NodeId) -> Option<Node> {
        self.graph.get(&id).cloned()
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = &Node> {
        self.graph.values()
    }

    pub fn len(&self) -> usize {
        self.graph.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Determines the shortest path between two nodes in the geograph
    /// using Dijsktra's algorithm and the Haversine formula.
    fn dijsktra(&self, origin: NodeId, destination: NodeId) -> Option<Vec<NodeId>> {
        let mut queue = BinaryHeap::new();
        let mut distances: HashMap<NodeId, Distance> = HashMap::new();
        let mut previous: HashMap<NodeId, NodeId> = HashMap::new();

        // Initialize distances and queue
        self.graph.keys().for_each(|&node_id| {
            let distance = if node_id == origin {
                Distance(0.0)
            } else {
                Distance(f64::INFINITY)
            };
            distances.insert(node_id, distance);
            queue.push(Reverse((distance, node_id)));
        });

        while let Some(Reverse((Distance(dist), current))) = queue.pop() {
            // Early exit if the destination node is reached
            if current == destination {
                let mut path = Vec::new();
                let mut step = destination;
                while step != origin {
                    if let Some(&prev) = previous.get(&step) {
                        path.push(step);
                        step = prev;
                    } else {
                        return None; // No path found
                    }
                }
                path.push(origin);
                path.reverse();
                return Some(path);
            }

            // Process each neighbor
            if let Some(node) = self.graph.get(&current) {
                for &neighbor_id in node.waypoints.iter() {
                    let neighbor = self.get(neighbor_id).expect("Missing neighbor");
                    let additional_distance = node.haversine(neighbor);
                    let total_distance = Distance(dist + additional_distance);

                    let neighbor_distance = *distances
                        .get(&neighbor_id)
                        .unwrap_or(&Distance(f64::INFINITY));

                    if total_distance < neighbor_distance {
                        distances.insert(neighbor_id, total_distance);
                        previous.insert(neighbor_id, current);
                        queue.push(Reverse((total_distance, neighbor_id)));
                    }
                }
            }
        }

        None // If no path is found
    }
}

#[macro_export]
macro_rules! build_geograph_mod {
    ($geograph:expr) => {
        lazy_static! {
            pub static ref GEOGRAPH: Geograph = $geograph.into();
        }

        pub fn geograph() -> &'static Geograph {
            &GEOGRAPH
        }

        pub fn shortest_path(
            origin: &impl Geolocalizable,
            destination: &impl Geolocalizable,
        ) -> ShortestPath {
            GEOGRAPH.shortest_path(origin, destination)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn geograph_fixture() -> Geograph {
        let mut geograph = Geograph::new("Test Geograph");
        geograph
            .add(Node::new(0, (0.0, 0.0), vec![1]))
            .add(Node::new(1, (1.0, 1.0), vec![0, 2, 3, 4]))
            .add(Node::new(2, (2.0, 2.0), vec![1, 3, 5]))
            .add(Node::new(3, (3.0, 3.0), vec![1, 2, 3, 4, 5]))
            .add(Node::new(4, (4.0, 4.0), vec![1, 3, 4, 5]))
            .add(Node::new(5, (5.0, 5.0), vec![3, 4]));

        geograph
    }

    #[test]
    fn test_add() {
        let mut geograph = geograph_fixture();
        geograph.add(Node::new(6, (6.0, 6.0), vec![]));

        assert_eq!(geograph.len(), 7);
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
        let non_existent_node = geograph.get(100);

        assert!(non_existent_node.is_none());
    }

    #[test]
    fn test_nodes() {
        let geograph = geograph_fixture();
        let nodes: Vec<&Node> = geograph.iter_nodes().collect();

        assert_eq!(nodes.len(), 6);
    }

    #[test]
    fn test_closest() {
        let geograph = geograph_fixture();
        let origin = geograph.get(0).unwrap();
        let very_close_to_origin = Node::new(6, (0.01, 0.01), vec![1]);

        let closest = geograph.closest(&very_close_to_origin).unwrap();

        assert_eq!(closest.id, origin.id);
    }

    #[test]
    fn test_shortest_path() {
        let geograph = geograph_fixture();

        let path = geograph.dijsktra(0, 5).unwrap();
        assert_eq!(path, vec![0, 1, 2, 5]);

        let path = geograph.dijsktra(2, 0).unwrap();
        assert_eq!(path, vec![2, 1, 0]);

        let path = geograph.dijsktra(4, 0).unwrap();
        assert_eq!(path, vec![4, 1, 0]);

        let path = geograph.dijsktra(5, 0).unwrap();
        assert_eq!(path, vec![5, 4, 1, 0]);
    }
}
