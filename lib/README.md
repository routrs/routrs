# routrs
Geograph-based shortest distance calculation for Rust.

The distance calculation functions in this library are designed to work with various types of geographs (maritime, highway, or railway).

When provided with origin and destination coordinates, the library will attempt to find the nearest points on the respective geograph if the given coordinates are not directly on it.

Distance calculations are performed using the [haversine](https://en.wikipedia.org/wiki/Haversine_formula) formula between nodes in the geograph, ensuring accurate representations of distances on a spherical surface. Then, the shortest path is calculated using the [Dijkstra's algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm).


It's important to note that if no path connecting the two coordinates is found in the geograph, the function will return the direct haversine distance between the two points, along with a path containing only the origin and destination points.

Please be aware that this library is primarily intended for generating realistic distance estimates and paths for visualization or estimation purposes. It is not designed for precise navigation or routing in real-world scenarios. The calculated routes and distances should be used for approximations and visual representations rather than as definitive guidance for actual travel or transportation planning.

## Available Geographs

- **Maritime**: based on the [MARNET](http://marnetproject.eu/) geograph.
- **Highways**: based on the [OpenStreetMap](https://www.openstreetmap.org/) highways database
- **Railways**: based on the [OpenStreetMap](https://www.openstreetmap.org/) railways database

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
routrs = { version = "2.0.0", features = ["all"] }
```

Available features:

- `maritime`: enables maritime geograph
- `highways`: enables highways geograph
- `railways`: enables railways geograph
- `json`: enables JSON parsing of geograph data 
- `concurrency`: enables parallel path calculation
- `all` enables all features

## Usage

```rust
use routrs::prelude::*;

// If you want to use the highways geograph:
use routrs::highways::GEOGRAPH as highways;

let from: Geoloc = (31.33068357, 120.902694); // Kunshan, Suzhou, Jiangsu, China
let to: Geoloc = (31.05287995, 121.2232226); // Songjiang District, Shanghai, China
let (distance, path, path_type) = highways::shortest_path(&from, &to);

println!("Distance: {} km", distance); // Distance: 57.237115955889074 km
println!("Nodes in path: {}", path.len()); // Nodes in path: 39
println!("Path type: {}", path_type); // Path type: ViaWaypoints

// If you want to use the maritime geograph:
use routrs::maritime::GEOGRAPH as maritime;

let (distance, path, path_type) = maritime::shortest_path(&from, &to);

// If you want to use the railway geograph:
use routrs::railways::GEOGRAPH as railways;

let (distance, path, path_type) = railways::shortest_path(&from, &to);
```

## Concurrent Path Calculation
Use the `concurrency` feature to enable concurrent path calculation, which is
based on the [rayon](https://crates.io/crates/rayon) crate.

You can pass a vector of legs to the `par_distance` method to calculate the
distances and path types concurrently.

```rust
use routrs::concurrency::*;
use routrs::highways::GEOGRAPH as highways;
use routrs::prelude::*;

let legs: Vec<Leg<Geoloc>> = vec![
    Leg((31.33068357, 120.902694), (31.05287995, 121.2232226)),
    // ... add more legs here
];

let results = highways.par_distance(&legs);

// results is a vector of distances and path types
```

## Custom Geographs from JSON data
You can use the `json` feature to create your own geograph from a JSON file.

```rust
use routrs::prelude::*;
use routrs::json::*;

let json_data = r#"{"geograph": "marnet", "nodes": [{"id": 0, "coordinates": [179.5, 51.3], "waypoints": [1, 3684, 5945, 5257, 10859, 3512, 6947, 8385, 2446, 2222]}]}"#;

let json_geograph: JsonGeograph =
    serde_json::from_str(json_data).expect("Failed to parse JSON");

let graph: Geograph = json_geograph.into();

// You can now use the graph for distance calculations
let from: Geoloc = (179.5, 51.3);
let to: Geoloc = (179.5, 51.3);
let (distance, path, path_type) = graph.shortest_path(&from, &to);
```

The JSON data should be a valid routrs Geograph structure:
- `name`: name of geograph
- `nodes`: list of nodes, with `id`, `coordinates`, and `waypoints` fields.
  - `id`: node id as usize (must be unique)
  - `coordinates`: node coordinates as [lng, lat]
  - `waypoints`: list of waypoints ids that are connected to the node

Example:
```json
{
    "geograph": "my_custom_geograph",
    "nodes": [
        {
          "id": 0, 
          "coordinates": [179.5, 51.3], 
          "waypoints": [1, 2, 4, 6, 72, 801]
        },
        {
          "id": 1, 
          "coordinates": [177.2, 52.1], 
          "waypoints": [ 1, 5, 7, 802, 25 ]
        }
        // ...
    ]
}
``` 

## Examples
See the examples folder for more usage examples.

To run the examples:
```bash
cargo run -p routrs_examples --bin maritime
cargo run -p routrs_examples --bin highways
cargo run -p routrs_examples --bin railways
cargo run -p routrs_examples --bin concurrency
```

## Python Bindings
Python bindings are available in the [routrs](https://github.com/routrs/pyroutrs) pypi package.

## License
MIT

## Contributing
Contributions are welcome! Please open an issue or submit a PR.
Contact Carlo Casorzo at carlo[a]deepblu.dev or in X [@deepbludev](https://x.com/deepbludev)

## Development
```bash
cargo build -p routrs
cargo test -p routrs
cargo test -p routrs_examples
```

## Attributions
Inspired by [scgraph](https://github.com/connor-makowski/scgraph) and [searoute](https://github.com/genthalili/searoute-py), including the use their datasets, which have been modified to work with this package.



