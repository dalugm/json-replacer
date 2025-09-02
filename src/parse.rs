use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

pub mod object_entity;
pub mod payload;
pub mod reference;
pub mod response;

#[derive(Deserialize)]
pub struct ObjectAttribute {
    data_type: String,
    name: String,
    picklist_options: Vec<PicklistOption>,
}

#[derive(Deserialize)]
struct PicklistOption {
    id: String,
    name: String,
}

#[derive(Deserialize)]
pub struct ObjectEntity {
    attributes: HashMap<String, Value>,
}

#[derive(Deserialize)]
struct SearchQuery {
    search_query_groups: Vec<SearchQueryGroup>,
}

#[derive(Deserialize)]
struct SearchQueryGroup {
    operator: SearchQueryGroupOperator,
    search_query_conditions: Option<Vec<SearchQueryCondition>>,
    children: Option<Vec<SearchQueryGroup>>,
}

#[derive(Deserialize)]
struct SearchQueryCondition {
    operator: SearchQueryConditionOperator,
    object_attribute_id: String,
    value: Option<Value>,
}

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum SearchQueryGroupOperator {
    And,
    Or,
    Not,
}

#[derive(Deserialize)]
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
        .map(|(key, value)| (parse_oa_uuid(&key), value))
        .collect()
}

fn parse_oa_uuid(key: &str) -> String {
    key.trim_start_matches("oa_").replace("_", "-")
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
    let name = oa.name.clone();

    match oa.data_type.as_str() {
        "picklist" => match value {
            Value::Null => {
                println!(
                    "Missing picklist value for id: {key}, which oa name is {}",
                    name
                );
                (name, value)
            }
            _ => {
                let picklist_option = oa.picklist_options.iter().find(|option| option.id == value);

                match picklist_option {
                    Some(option) => (name, serde_json::Value::String(option.name.clone())),
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
