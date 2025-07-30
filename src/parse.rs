use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

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
struct ObjectEntity {
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
