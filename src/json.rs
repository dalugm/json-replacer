use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

pub mod payload;
pub mod reference;
pub mod response;

#[derive(Debug, Deserialize)]
struct PicklistOptionAttribute {
    name: String,
}

#[derive(Debug, Deserialize)]
struct PicklistOption {
    id: String,
    attributes: PicklistOptionAttribute,
}

#[derive(Debug, Deserialize)]
struct Attribute {
    name: String,
    data_type: String,
}

#[derive(Debug, Deserialize)]
pub struct ObjectAttribute {
    attributes: Attribute,
    included: Vec<PicklistOption>,
}

#[derive(Debug, Deserialize)]
struct ObjectEntity {
    attributes: HashMap<String, Value>,
}
