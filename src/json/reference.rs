use anyhow::Result;
use std::{collections::HashMap, fs::File, io::BufReader};

use super::ObjectAttribute;

pub fn parse_reference(file_path: String) -> Result<HashMap<String, ObjectAttribute>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let reference_hashmap: HashMap<String, ObjectAttribute> = serde_json::from_reader(reader)?;

    Ok(reference_hashmap)
}
