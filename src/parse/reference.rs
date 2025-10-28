use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

use super::{ObjectAttribute, ObjectAttributeDataType, PicklistOption};

#[derive(Deserialize)]
pub struct ObjectAttributesRaw {
    data: Vec<ObjectAttributesData>,
    included: HashMap<String, ObjectAttributesIncluded>,
}

#[derive(Deserialize)]
struct ObjectAttributesData {
    id: String,
    attributes: ObjectAttributesDataAttributes,
    relationships: ObjectAttributesDataRelationships,
}

#[derive(Deserialize)]
struct ObjectAttributesDataAttributes {
    name: String,
    data_type: ObjectAttributeDataType,
}

#[derive(Deserialize)]
struct ObjectAttributesDataRelationships {
    picklist_options: Relationship<Vec<RelationshipDataPicklistOption>>,
}

#[derive(Deserialize)]
struct ObjectAttributesIncluded {
    id: String,
    attributes: ObjectAttributesIncludedAttributes,
}

#[derive(Deserialize)]
struct ObjectAttributesIncludedAttributes {
    name: String,
}

#[derive(Deserialize)]
struct Relationship<T> {
    data: Option<T>,
}

#[derive(Deserialize)]
struct RelationshipDataPicklistOption {
    id: String,
}

pub fn parse(raw_data: ObjectAttributesRaw) -> Result<HashMap<String, ObjectAttribute>> {
    let mut map = HashMap::with_capacity(raw_data.data.len());

    for oa in &raw_data.data {
        let picklist_options = oa
            .relationships
            .picklist_options
            .data
            .as_deref()
            .unwrap_or_default()
            .iter()
            .map(|option| {
                raw_data
                    .included
                    .get(&option.id)
                    .map(|included| PicklistOption {
                        id: included.id.clone(),
                        name: included.attributes.name.clone(),
                    })
                    .unwrap_or_else(|| PicklistOption {
                        id: option.id.to_string(),
                        name: "not_found".to_string(),
                    })
            })
            .collect();

        map.entry(oa.id.to_string())
            .or_insert_with(|| ObjectAttribute {
                data_type: oa.attributes.data_type,
                name: oa.attributes.name.clone(),
                picklist_options,
            });
    }

    Ok(map)
}
