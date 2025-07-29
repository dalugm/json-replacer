use std::{collections::HashMap, fs::File, io::BufReader};

use anyhow::Result;
use serde::Deserialize;
use serde_json::Value;

use super::{ObjectAttribute, SearchQuery};

#[derive(Debug, Deserialize)]
pub struct Payload {
    object_attribute_ids: Vec<String>,
    search_query: SearchQuery,
}

pub fn parse_payload(
    file_path: String,
    hashmap: &HashMap<String, ObjectAttribute>,
) -> Result<HashMap<String, Value>> {
    let file = File::open(file_path).expect("failed to open payload");
    let reader = BufReader::new(file);
    let payload: Payload = serde_json::from_reader(reader).expect("failed to parse payload file");

    let mut map = HashMap::new();

    let names = payload
        .object_attribute_ids
        .into_iter()
        .map(|id| match hashmap.get(&id) {
            Some(oa) => oa.attributes.name.clone(),
            None => "not found".to_string(),
        })
        .collect();

    map.insert("names".to_string(), names);
    map.insert(
        "search_query".to_string(),
        serde_json::Value::String("Not implemented yet.".to_string()),
    );

    Ok(map)
}
