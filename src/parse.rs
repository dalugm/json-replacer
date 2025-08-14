use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

pub mod object_entity;
pub mod payload;
pub mod reference;
pub mod response;

#[derive(Debug, Deserialize)]
pub struct ObjectAttribute {
    attributes: Attribute,
    included: Vec<PicklistOption>,
}

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
pub struct ObjectEntity {
    attributes: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
    search_query_groups: Vec<SearchQueryGroup>,
}

#[derive(Debug, Deserialize)]
struct SearchQueryGroup {
    operator: SearchQueryGroupOperator,
    search_query_conditions: Option<Vec<SearchQueryCondition>>,
    children: Option<Vec<SearchQueryGroup>>,
}

#[derive(Debug, Deserialize)]
struct SearchQueryCondition {
    operator: SearchQueryConditionOperator,
    object_attribute_id: String,
    value: Option<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum SearchQueryGroupOperator {
    And,
    Or,
    Not,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum SearchQueryConditionOperator {
    Equal,
    NotEqual,
    Contain,
    NotContain,
    IsPresent,
    IsBlank,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    Between,
    Today,
    BeforeToday,
    AfterToday,
    ThisWeek,
    BeforeThisWeek,
    AfterThisWeek,
    ThisMonth,
    BeforeThisMonth,
    AfterThisMonth,
    ThisQuarter,
    BeforeThisQuarter,
    AfterThisQuarter,
    ThisYear,
    BeforeThisYear,
    AfterThisYear,
    AnyOf,
    NoneOf,
    IsTrue,
    IsFalse,
    Address,
}

/// Keep attributes which start with `oa_` only, and change its value to uuid.
fn extract_entity_oa_attributes(entity: &ObjectEntity) -> HashMap<String, Value> {
    entity
        .attributes
        .clone()
        .into_iter()
        .filter(|(key, _)| key.starts_with("oa_"))
        .map(|(key, value)| (key.trim_start_matches("oa_").replace("_", "-"), value))
        .collect()
}

fn convert_entity_uuid_to_value(
    entity: HashMap<String, Value>,
    hashmap: &HashMap<String, ObjectAttribute>,
) -> HashMap<String, Value> {
    entity
        .into_iter()
        .map(|(key, value)| match hashmap.get(&key) {
            Some(oa) => process_entity_attribute(oa, key, value),
            None => {
                println!("Unknown object attribute id: {key}");
                (key, value)
            }
        })
        .collect()
}

fn process_entity_attribute(oa: &ObjectAttribute, key: String, value: Value) -> (String, Value) {
    let name = oa.attributes.name.clone();

    match oa.attributes.data_type.as_str() {
        "picklist" => match value {
            Value::Null => {
                println!(
                    "Missing picklist value for id: {key}, which oa name is {}",
                    oa.attributes.name
                );
                (name, value)
            }
            _ => {
                let picklist_option = oa.included.iter().find(|option| option.id == value);

                match picklist_option {
                    Some(option) => (
                        name,
                        serde_json::Value::String(option.attributes.name.clone()),
                    ),
                    None => {
                        println!("Picklist option not found for id: {key}");
                        (name, "not found".into())
                    }
                }
            }
        },
        _ => (name, value),
    }
}
