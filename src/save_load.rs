use bevy::prelude::*;
use serde::{Deserialize, Serialize, Serializer};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Result;
use std::path::Path;

pub fn serialize_to_file<T: Serialize>(object: &T, name: &str) -> Result<()> {
    let serialized = dbg!(serde_json::to_string(&object))?;
    let filename = name.to_owned() + ".json";
    let path = Path::new(&filename);

    fs::write(path, serialized)?;
    Ok(())
}

pub fn deserialize_from_file<T: Deserialize>(name: &str) -> Result<T> {
    let filename = name.to_owned() + ".json";
    let path = Path::new(&filename);
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);

    let mut serialized = String::new();

    buf_reader.read_to_string(&mut serialized)?;

    let output: T = serde_json::from_str(&serialized)?;

    Ok(output)
}
