use anyhow::Result;
use std::{collections::HashMap, fs::File, io::BufReader};

use serde::Deserialize;
use serde_json::Value;

use super::{ObjectAttribute, ObjectEntity, convert_raw_entity};

#[derive(Deserialize)]
struct Response {
    data: Vec<ObjectEntity>,
}

pub fn parse(
    input: String,
    hashmap: &HashMap<String, ObjectAttribute>,
) -> Result<Vec<HashMap<String, Value>>> {
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
) -> Result<Vec<HashMap<String, Value>>> {
    let file = File::open(file_path).expect("failed to open response file");
    let reader = BufReader::new(file);
    let response: Response =
        serde_json::from_reader(reader).expect("failed to parse response file");

    Ok(parse_response(response, hashmap))
}

fn parse_string(
    json_str: String,
    hashmap: &HashMap<String, ObjectAttribute>,
) -> Result<Vec<HashMap<String, Value>>> {
    let response: Response =
        serde_json::from_str(&json_str).expect("failed to parse response file");

    Ok(parse_response(response, hashmap))
}

fn parse_response(
    response: Response,
    hashmap: &HashMap<String, ObjectAttribute>,
) -> Vec<HashMap<String, Value>> {
    response
        .data
        .iter()
        .map(|entity| convert_raw_entity(entity.attributes.clone(), hashmap))
        .collect()
}
