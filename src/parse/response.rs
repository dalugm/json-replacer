use anyhow::Result;
use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

use super::{ObjectAttribute, ObjectEntity, convert_raw_entity};

#[derive(Deserialize)]
pub struct Response {
    data: Vec<ObjectEntity>,
}

pub fn parse(
    response: Response,
    hashmap: &HashMap<String, ObjectAttribute>,
) -> Result<Vec<HashMap<String, Value>>> {
    let result = response
        .data
        .iter()
        .map(|entity| convert_raw_entity(entity.attributes.clone(), hashmap))
        .collect();

    Ok(result)
}
