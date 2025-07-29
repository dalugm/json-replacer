use anyhow::Result;
use std::{collections::HashMap, fs::File, io::BufReader};

use serde::Deserialize;
use serde_json::Value;

use super::{ObjectAttribute, ObjectEntity};

#[derive(Debug, Deserialize)]
struct Response {
    data: Vec<ObjectEntity>,
}

fn extract_oa_attributes(entities: Vec<ObjectEntity>) -> Vec<HashMap<String, Value>> {
    entities
        .into_iter()
        .map(|entity| {
            entity
                .attributes
                .into_iter()
                .filter(|(key, _)| key.starts_with("oa_"))
                .map(|(key, value)| (key.trim_start_matches("oa_").replace("_", "-"), value))
                .collect::<HashMap<String, Value>>()
        })
        .collect::<Vec<HashMap<String, Value>>>()
}

fn process_oa_attributes(
    oa_attributes: Vec<HashMap<String, Value>>,
    oa_id_hashmap: &HashMap<String, ObjectAttribute>,
) -> Result<Vec<HashMap<String, Value>>> {
    oa_attributes
        .into_iter()
        .map(|attrs| process_single_attribute(attrs, oa_id_hashmap))
        .collect()
}

fn process_single_attribute(
    mut oa_attribute: HashMap<String, Value>,
    oa_id_hashmap: &HashMap<String, ObjectAttribute>,
) -> Result<HashMap<String, Value>> {
    let result: HashMap<String, Value> = oa_attribute
        .drain()
        .map(|(key, value)| match oa_id_hashmap.get(&key) {
            Some(oa) => process_known_attribute(oa, key, value),
            None => {
                println!("Unknown object attribute id: {key}");
                Ok((key, value))
            }
        })
        .collect::<Result<_>>()?;

    Ok(result)
}

fn process_known_attribute(
    oa: &ObjectAttribute,
    key: String,
    value: Value,
) -> Result<(String, Value)> {
    let name = oa.attributes.name.clone();

    if oa.attributes.data_type == "picklist" {
        match value {
            Value::Null => {
                println!(
                    "Missing picklist value for id: {key}, which oa name is {}",
                    oa.attributes.name
                );
                Ok((name, value))
            }
            _ => {
                let picklist_option = oa.included.iter().find(|option| option.id == value);

                match picklist_option {
                    Some(option) => Ok((
                        name,
                        serde_json::Value::String(option.attributes.name.clone()),
                    )),
                    None => {
                        println!("Picklist option not found for id: {key}");
                        Ok((name, "not found".into()))
                    }
                }
            }
        }
    } else {
        Ok((name, value))
    }
}

pub fn parse_resposne(
    file_path: String,
    hashmap: &HashMap<String, ObjectAttribute>,
) -> Result<Vec<HashMap<String, Value>>> {
    let file = File::open(file_path).expect("failed to open source");
    let reader = BufReader::new(file);
    let response: Response = serde_json::from_reader(reader).expect("Failed to parse response");

    let oa_attributes = extract_oa_attributes(response.data);

    let object_entities: Vec<HashMap<String, Value>> =
        process_oa_attributes(oa_attributes, hashmap)?;

    Ok(object_entities)
}
