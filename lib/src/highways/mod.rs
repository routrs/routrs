use crate::{build_geograph_mod, json, prelude::*};
use lazy_static::lazy_static;
use routrs_highways_dataset;

build_geograph_mod!(serde_json::from_str::<json::JsonGeograph>(
    &routrs_highways_dataset::HIGHWAYS_JSON
)
.expect("Failed to parse Highways Geograph JSON"));
