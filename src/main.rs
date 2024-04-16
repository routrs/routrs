use routrs::geograph::json::JsonGeograph;
fn main() {
    let json_data = r#"{"geograph": "marnet", "nodes": [{"id": 0, "coordinates": [179.5, 51.3], "waypoints": [1, 3684, 5945, 5257, 10859, 3512, 6947, 8385, 2446, 2222]}]}"#;
    let parsed_data: JsonGeograph = serde_json::from_str(json_data).expect("Failed to parse JSON");
    println!("{:#?}", parsed_data);
}
