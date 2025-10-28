use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

use super::{ObjectAttribute, convert_raw_entity};

type ObjectEntity = HashMap<String, Value>;

pub fn parse(
    entity: ObjectEntity,
    hashmap: &HashMap<String, ObjectAttribute>,
) -> Result<ObjectEntity> {
    Ok(convert_raw_entity(entity, hashmap))
}
