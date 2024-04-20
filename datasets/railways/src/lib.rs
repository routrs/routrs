use lazy_static::lazy_static;

use flate2::read::GzDecoder;
use std::io::Read;

macro_rules! load_geograph_json {
    ($filename:expr) => {{
        let compressed_geograph_json = include_bytes!($filename);
        let decoder = GzDecoder::new(compressed_geograph_json.as_ref());
        let mut geograph_json = String::new();
        let mut reader = std::io::BufReader::new(decoder);
        reader
            .read_to_string(&mut geograph_json)
            .expect("Failed to decompress geograph file");

        geograph_json
    }};
}

lazy_static! {
    pub static ref RAILWAYS_JSON: String = load_geograph_json!("../data/railways.json.gz");
}
