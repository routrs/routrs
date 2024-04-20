use crate::{build_geograph_mod, json, prelude::*};
use lazy_static::lazy_static;
use routrs_maritime_dataset;

build_geograph_mod!(serde_json::from_str::<json::JsonGeograph>(
    &routrs_maritime_dataset::MARITIME_JSON
)
.expect("Failed to parse Maritime Geograph JSON"));
