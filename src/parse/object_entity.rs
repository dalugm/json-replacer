use anyhow::Result;
use serde_json::Value;
use std::{collections::HashMap, fs::File, io::BufReader};

use super::{ObjectAttribute, convert_raw_entity};

pub fn parse(
    input: String,
    hashmap: &HashMap<String, super::ObjectAttribute>,
) -> Result<HashMap<String, Value>> {
    let is_str = input.starts_with("{");

    if is_str {
        parse_string(input, hashmap)
    } else {
        parse_file(input, hashmap)
    }
}

fn parse_file(
    file_path: String,
    hashmap: &HashMap<String, ObjectAttribute>,
) -> Result<HashMap<String, Value>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let input: HashMap<String, Value> = serde_json::from_reader(reader)?;

    Ok(parse_object_entity(input, hashmap))
}

fn parse_string(
    string: String,
    hashmap: &HashMap<String, ObjectAttribute>,
) -> Result<HashMap<String, Value>> {
    let input: HashMap<String, Value> = serde_json::from_str(&string)?;

    Ok(parse_object_entity(input, hashmap))
}

fn parse_object_entity(
    entity: HashMap<String, Value>,
    hashmap: &HashMap<String, ObjectAttribute>,
) -> HashMap<String, Value> {
    convert_raw_entity(entity, hashmap)
}
