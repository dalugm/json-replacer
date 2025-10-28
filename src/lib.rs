mod parse;

#[cfg(target_arch = "wasm32")]
mod wasm;

use std::collections::HashMap;

use anyhow::Result;
use serde_json::Value;

use parse::{
    object_entity::parse as parse_entity,
    payload::parse as parse_payload,
    reference::{ObjectAttributesRaw, parse as parse_reference},
    response::parse as parse_response,
};

pub use parse::{ObjectAttribute, payload::Payload, response::Response};

pub fn preprocess_reference(
    reference: ObjectAttributesRaw,
) -> Result<HashMap<String, ObjectAttribute>> {
    parse_reference(reference)
}

pub fn process_payload(
    oa_id_hashmap: &HashMap<String, ObjectAttribute>,
    payload: Payload,
) -> Result<HashMap<String, Value>> {
    parse_payload(payload, oa_id_hashmap)
}

pub fn process_response(
    oa_id_hashmap: &HashMap<String, ObjectAttribute>,
    response: Response,
) -> Result<Vec<HashMap<String, Value>>> {
    parse_response(response, oa_id_hashmap)
}

pub fn process_object_entity(
    oa_id_hashmap: &HashMap<String, ObjectAttribute>,
    entity: HashMap<String, Value>,
) -> Result<HashMap<String, Value>> {
    parse_entity(entity, oa_id_hashmap)
}
