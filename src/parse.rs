pub mod object_entity;
pub mod payload;
pub mod reference;
pub mod response;

use std::{collections::HashMap, fmt::Display};

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct ObjectAttribute {
    data_type: ObjectAttributeDataType,
    name: String,
    picklist_options: Vec<PicklistOption>,
}

#[derive(Deserialize, PartialEq, Copy, Clone)]
#[serde(rename_all = "snake_case")]
enum ObjectAttributeDataType {
    Address,
    Boolean,
    Currency,
    Date,
    Datetime,
    Email,
    EncryptedString,
    File,
    Float,
    Id,
    Integer,
    NestedForm,
    Number,
    Percent,
    Phone,
    Picklist,
    Reference,
    String,
    Text,
    Textarea,
    Richtextarea,
}

impl Display for ObjectAttributeDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectAttributeDataType::Address => write!(f, "address"),
            ObjectAttributeDataType::Boolean => write!(f, "boolean"),
            ObjectAttributeDataType::Currency => write!(f, "currency"),
            ObjectAttributeDataType::Date => write!(f, "date"),
            ObjectAttributeDataType::Datetime => write!(f, "datetime"),
            ObjectAttributeDataType::Email => write!(f, "email"),
            ObjectAttributeDataType::EncryptedString => write!(f, "encrypted_string"),
            ObjectAttributeDataType::File => write!(f, "file"),
            ObjectAttributeDataType::Float => write!(f, "float"),
            ObjectAttributeDataType::Id => write!(f, "id"),
            ObjectAttributeDataType::Integer => write!(f, "integer"),
            ObjectAttributeDataType::NestedForm => write!(f, "nested_form"),
            ObjectAttributeDataType::Number => write!(f, "number"),
            ObjectAttributeDataType::Percent => write!(f, "percent"),
            ObjectAttributeDataType::Phone => write!(f, "phone"),
            ObjectAttributeDataType::Picklist => write!(f, "picklist"),
            ObjectAttributeDataType::Reference => write!(f, "reference"),
            ObjectAttributeDataType::String => write!(f, "string"),
            ObjectAttributeDataType::Text => write!(f, "text"),
            ObjectAttributeDataType::Textarea => write!(f, "textarea"),
            ObjectAttributeDataType::Richtextarea => write!(f, "rich_textarea"),
        }
    }
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

fn parse_oa_uuid(key: &str) -> String {
    key.trim_start_matches("oa_").replace("_", "-")
}

fn process_entity_attribute(
    oa: &ObjectAttribute,
    key: String,
    value: Value,
    hashmap: &HashMap<String, ObjectAttribute>,
) -> (String, Value) {
    let name = format!("{} ({})", oa.name, oa.data_type);

    match oa.data_type {
        ObjectAttributeDataType::Picklist => match value {
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
        ObjectAttributeDataType::NestedForm => match value {
            Value::Null => (name, value),
            _ => {
                let nested_form_value: HashMap<String, payload::Payload> =
                    serde_json::from_value(value).expect("failed to parse nested_form");

                let values: Vec<HashMap<String, Value>> = nested_form_value
                    .into_values()
                    .map(|value| match value.object_entity_attribute_values {
                        Some(values) => convert_raw_entity(values, hashmap),
                        None => HashMap::new(),
                    })
                    .collect();

                let json = serde_json::to_value(values).expect("failed to convert values to json");

                (name, json)
            }
        },
        _ => (name, value),
    }
}

fn convert_raw_entity(
    entity: HashMap<String, Value>,
    hashmap: &HashMap<String, ObjectAttribute>,
) -> HashMap<String, Value> {
    entity
        .into_iter()
        .filter(|(key, _)| key.starts_with("oa_"))
        .map(|(key, value)| (parse_oa_uuid(&key), value))
        .map(|(key, value)| match hashmap.get(&key) {
            Some(oa) => process_entity_attribute(oa, key, value, hashmap),
            None => {
                println!("Unknown object attribute id: {key}");
                (key, value)
            }
        })
        .collect()
}
