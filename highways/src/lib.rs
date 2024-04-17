use lazy_static::lazy_static;

use flate2::read::GzDecoder;
use routrs::prelude::*;
use std::io::Read;

macro_rules! load_geograph {
    ($filename:expr) => {{
        let compressed_data = include_bytes!($filename);
        let decoder = GzDecoder::new(compressed_data.as_ref());
        let mut decompressed_data = String::new();
        let mut reader = std::io::BufReader::new(decoder);
        reader
            .read_to_string(&mut decompressed_data)
            .expect("Failed to decompress geograph file");
        serde_json::from_str::<JsonGeograph>(&decompressed_data)
            .expect("Failed to parse Geograph JSON")
            .into()
    }};
}

lazy_static! {
    pub static ref HIGHWAYS: Geograph = load_geograph!("../data/highways.json.gz");
}
