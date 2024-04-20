use crate::geograph::{json::JsonGeograph, Geograph};
use lazy_static::lazy_static;
use routrs_highways_dataset::HIGHWAYS_JSON;

lazy_static! {
    pub static ref GEOGRAPH: Geograph = serde_json::from_str::<JsonGeograph>(&HIGHWAYS_JSON)
        .expect("Failed to parse Geograph JSON")
        .into();
}
