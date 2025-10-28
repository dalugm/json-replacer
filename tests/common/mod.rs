use std::{collections::HashMap, fs};

use json_replacer::{ObjectAttribute, preprocess_reference};

pub fn setup() -> HashMap<String, ObjectAttribute> {
    let content = fs::read_to_string("tests/oa.json").unwrap();
    let reference = serde_json::from_str(&content).unwrap();
    preprocess_reference(reference).expect("failed to parse reference file")
}
