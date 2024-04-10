use bevy::prelude::*;
use serde::{Serialize, Serializer};
use std::path::Path;

pub fn serialize_to_file<T: Serialize>(object: T) {
    let serialized = dbg!(serde_json::to_string(&object).unwrap());
}
