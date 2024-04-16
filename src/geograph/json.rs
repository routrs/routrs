use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonGeograph {
    pub geograph: String,
    pub nodes: Vec<JsonGeographNode>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonGeographNode {
    id: i32,
    coordinates: Vec<f64>,
    waypoints: Vec<i32>,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_parses_raw_json() {
        let json_data = r#"{"geograph": "marnet", "nodes": [{"id": 0, "coordinates": [179.5, 51.3], "waypoints": [1, 3684, 5945, 5257, 10859, 3512, 6947, 8385, 2446, 2222]}]}"#;
        let json_geograph: JsonGeograph =
            serde_json::from_str(json_data).expect("Failed to parse JSON");

        assert_eq!(json_geograph.geograph, "marnet");
        assert!(json_geograph.nodes.len() == 1);
    }
}
