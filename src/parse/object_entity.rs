use anyhow::Result;
use serde_json::Value;
use std::{collections::HashMap, fs::File, io::BufReader};

use super::{
    ObjectAttribute, ObjectEntity, convert_entity_uuid_to_value, extract_entity_oa_attributes,
};

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

    let input: ObjectEntity = serde_json::from_reader(reader)?;

    Ok(parse_object_entity(&input, hashmap))
}

fn parse_string(
    string: String,
    hashmap: &HashMap<String, ObjectAttribute>,
) -> Result<HashMap<String, Value>> {
    let input: ObjectEntity = serde_json::from_str(&string)?;

    Ok(parse_object_entity(&input, hashmap))
}

fn parse_object_entity(
    entity: &ObjectEntity,
    hashmap: &HashMap<String, ObjectAttribute>,
) -> HashMap<String, Value> {
    let entity = extract_entity_oa_attributes(entity);

    convert_entity_uuid_to_value(entity, hashmap)
}
