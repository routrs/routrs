use crate::geograph::{json::JsonGeograph, Geograph};
use lazy_static::lazy_static;
use routrs_railways_dataset::RAILWAYS_JSON;

lazy_static! {
    pub static ref GEOGRAPH: Geograph = serde_json::from_str::<JsonGeograph>(&RAILWAYS_JSON)
        .expect("Failed to parse Railways Geograph JSON")
        .into();
}
