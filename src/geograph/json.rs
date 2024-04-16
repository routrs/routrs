use serde::{Deserialize, Serialize};

use super::{Node, NodeId};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonGeograph {
    pub geograph: String,
    pub nodes: Vec<JsonNode>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonNode {
    pub id: NodeId,
    pub coordinates: Vec<f64>,
    pub waypoints: Vec<i32>,
}

impl From<JsonNode> for Node {
    fn from(node: JsonNode) -> Node {
        Node::new(
            node.id,
            (node.coordinates[0], node.coordinates[1]),
            node.waypoints,
        )
    }
}

#[cfg(test)]
mod test {
    use crate::geograph::Geolocalizable;

    use super::*;

    #[test]
    fn it_parses_raw_json() {
        let json_data = r#"{"geograph": "marnet", "nodes": [{"id": 0, "coordinates": [179.5, 51.3], "waypoints": [1, 3684, 5945, 5257, 10859, 3512, 6947, 8385, 2446, 2222]}]}"#;
        let json_geograph: JsonGeograph =
            serde_json::from_str(json_data).expect("Failed to parse JSON");

        assert_eq!(json_geograph.geograph, "marnet");
        assert!(json_geograph.nodes.len() == 1);
    }

    #[test]
    fn it_converts_into_geograph_node() {
        let node = JsonNode {
            id: 0,
            coordinates: vec![179.5, 51.3],
            waypoints: vec![1, 3684, 5945, 5257, 10859, 3512, 6947, 8385, 2446, 2222],
        };
        let geograph_node: Node = node.clone().into();

        assert_eq!(geograph_node.id, 0);
        assert_eq!(geograph_node.lat(), *node.coordinates.first().unwrap());
        assert_eq!(geograph_node.lng(), *node.coordinates.last().unwrap());
        assert_eq!(geograph_node.waypoints, node.waypoints);
    }
}
