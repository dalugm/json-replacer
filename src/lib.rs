use std::{collections::HashMap, error::Error, fs::File, io::BufReader};

use serde::Deserialize;
use serde_json::Value;

pub struct Config {
    response_file_path: String,
    oa_id_hashmap_file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let response_file_path = args[1].clone();
        let oa_id_hashmap_file_path = args[2].clone();

        Ok(Config {
            response_file_path,
            oa_id_hashmap_file_path,
        })
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct PicklistOptionAttribute {
    label: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct PicklistOption {
    id: String,
    attributes: PicklistOptionAttribute,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Attribute {
    label: String,
    name: String,
    data_type: String,
    reference_to: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ObjectAttribute {
    id: String,
    attributes: Attribute,
    included: Vec<PicklistOption>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ObjectEntity {
    id: String,
    attributes: HashMap<String, Value>,
}

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
) -> Result<Vec<HashMap<String, Value>>, Box<dyn Error>> {
    oa_attributes
        .into_iter()
        .map(|attrs| process_single_attribute(attrs, oa_id_hashmap))
        .collect()
}

fn process_single_attribute(
    mut oa_attribute: HashMap<String, Value>,
    oa_id_hashmap: &HashMap<String, ObjectAttribute>,
) -> Result<HashMap<String, Value>, Box<dyn Error>> {
    let result: HashMap<String, Value> = oa_attribute
        .drain()
        .map(|(key, value)| match oa_id_hashmap.get(&key) {
            Some(oa) => process_known_attribute(oa, key, value),
            None => {
                println!("Unknown object attribute id: {key}");
                Ok((key, value))
            }
        })
        .collect::<Result<_, Box<dyn Error>>>()?;

    Ok(result)
}

fn process_known_attribute(
    oa: &ObjectAttribute,
    key: String,
    value: Value,
) -> Result<(String, Value), Box<dyn Error>> {
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let hashmap_file = File::open(config.oa_id_hashmap_file_path).expect("failed to open file");
    let hashmap_reader = BufReader::new(hashmap_file);
    let oa_id_hashmap: HashMap<String, ObjectAttribute> =
        serde_json::from_reader(hashmap_reader).expect("failed to parse hashmap file");

    let response_file = File::open(config.response_file_path).expect("failed to open file");
    let response_reader = BufReader::new(response_file);
    let serialized_response: Response =
        serde_json::from_reader(response_reader).expect("Failed to parse response");

    let oa_attributes = extract_oa_attributes(serialized_response.data);

    let object_entities: Vec<HashMap<String, Value>> =
        process_oa_attributes(oa_attributes, &oa_id_hashmap)?;

    println!("{:#?}", object_entities);

    Ok(())
}
