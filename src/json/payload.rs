use std::{collections::HashMap, fs::File, io::BufReader};

use anyhow::Result;
use serde::Deserialize;

use super::ObjectAttribute;

#[derive(Debug, Deserialize)]
pub struct Payload {
    object_attribute_ids: Vec<String>,
}

pub fn parse_payload(
    file_path: String,
    hashmap: &HashMap<String, ObjectAttribute>,
) -> Result<Vec<HashMap<String, String>>> {
    let file = File::open(file_path).expect("failed to open source");
    let reader = BufReader::new(file);
    let payload: Payload = serde_json::from_reader(reader).expect("Failed to parse response");

    let id_map = payload
        .object_attribute_ids
        .into_iter()
        .map(|id| {
            let mut map = HashMap::new();
            let value = hashmap
                .get(&id)
                .map(|oa| oa.attributes.name.clone())
                .unwrap_or_else(|| "not found".to_string());
            map.insert(id, value);
            map
        })
        .collect();

    Ok(id_map)
}
