use crate::{build_geograph_mod, json, prelude::*};
use lazy_static::lazy_static;
use routrs_railways_dataset;

build_geograph_mod!(serde_json::from_str::<json::JsonGeograph>(
    &routrs_railways_dataset::RAILWAYS_JSON
)
.expect("Failed to parse Railways Geograph JSON"));
