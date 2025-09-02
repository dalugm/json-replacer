use std::collections::HashMap;

use json_replacer::parse::{ObjectAttribute, reference::parse};

pub fn setup() -> HashMap<String, ObjectAttribute> {
    parse("tests/oa.json".to_string()).expect("failed to parse reference file")
}
