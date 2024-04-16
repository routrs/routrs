use std::fs;

use routrs::geograph::json;
fn main() {
    let marnet_json_data = fs::read_to_string(format!("data/{}.json", "marnet"))
        .expect("Failed to read file data/marnet.json");

    let marnet_json: json::JsonGeograph =
        serde_json::from_str(&marnet_json_data).expect("Failed to parse JSON");

    println!("{:#?}", marnet_json);
}
