use std::{collections::HashMap, fs::File, io::BufReader};

use anyhow::Result;
use serde::Deserialize;
use serde_json::Value;

use crate::parse::SearchQueryGroup;

use super::{
    ObjectAttribute, SearchQuery, SearchQueryCondition, SearchQueryConditionOperator,
    SearchQueryGroupOperator,
};

#[derive(Debug, Deserialize)]
pub struct Payload {
    object_attribute_ids: Vec<String>,
    search_query: SearchQuery,
}

/// Transform picklist oa id to name.
fn process_picklist_oa_value(oa: &ObjectAttribute, value: Value) -> Value {
    match value {
        Value::String(option_id) => {
            let picklist_option = oa.included.iter().find(|option| option.id == option_id);

            match picklist_option {
                Some(option) => serde_json::Value::String(option.attributes.name.clone()),
                None => {
                    println!("Picklist option not found for id: {option_id}");
                    "not found picklist label".into()
                }
            }
        }
        Value::Array(option_ids) => option_ids
            .iter()
            .map(|option_id| {
                let picklist_option = oa.included.iter().find(|option| option.id == *option_id);

                match picklist_option {
                    Some(option) => serde_json::Value::String(option.attributes.name.clone()),
                    None => {
                        println!("Picklist option not found for id: {option_id}");
                        "not found picklist label".into()
                    }
                }
            })
            .collect(),
        _ => "picklist value is {option_id_value:#?}, which is not implemented yet.".into(),
    }
}

fn parse_search_query_group(
    group: SearchQueryGroup,
    hashmap: &HashMap<String, ObjectAttribute>,
) -> String {
    let mut expr = "(".to_owned();

    let operator = match group.operator {
        SearchQueryGroupOperator::And => "AND",
        SearchQueryGroupOperator::Or => "OR",
        SearchQueryGroupOperator::Not => "NOT",
    };

    expr.push_str(operator);

    if let Some(conditions) = group.search_query_conditions {
        let cond_expr = parse_search_query_conditions(conditions, hashmap);
        expr.push_str(&cond_expr);
    }

    if let Some(children) = group.children {
        let cond_expr = parse_search_query_children(children, hashmap);
        expr.push_str(&cond_expr);
    }

    expr.push(')');

    expr
}

fn parse_search_query_children(
    children: Vec<SearchQueryGroup>,
    hashmap: &HashMap<String, ObjectAttribute>,
) -> String {
    let mut expr = "(".to_owned();

    for child in children {
        expr.push_str(&parse_search_query_group(child, hashmap));
    }

    expr.push(')');

    expr
}

fn parse_search_query_conditions(
    conditions: Vec<SearchQueryCondition>,
    hashmap: &HashMap<String, ObjectAttribute>,
) -> String {
    let mut expr = "(".to_owned();

    for condition in conditions {
        expr.push('(');
        let operator = match condition.operator {
            SearchQueryConditionOperator::Equal => "equal",
            SearchQueryConditionOperator::NotEqual => "not_equal",
            SearchQueryConditionOperator::Contain => "contain",
            SearchQueryConditionOperator::NotContain => "not_contain",
            SearchQueryConditionOperator::IsPresent => "is_present",
            SearchQueryConditionOperator::IsBlank => "is_blank",
            SearchQueryConditionOperator::Greater => "greater_than",
            SearchQueryConditionOperator::GreaterOrEqual => "greater_than_equal",
            SearchQueryConditionOperator::Less => "less_than",
            SearchQueryConditionOperator::LessOrEqual => "less_than_equal",
            SearchQueryConditionOperator::Between => "between",
            SearchQueryConditionOperator::Today => "today",
            SearchQueryConditionOperator::BeforeToday => "before_today",
            SearchQueryConditionOperator::AfterToday => "after_today",
            SearchQueryConditionOperator::ThisWeek => "this_week",
            SearchQueryConditionOperator::BeforeThisWeek => "before_this_week",
            SearchQueryConditionOperator::AfterThisWeek => "after_this_week",
            SearchQueryConditionOperator::ThisMonth => "this_month",
            SearchQueryConditionOperator::BeforeThisMonth => "before_this_month",
            SearchQueryConditionOperator::AfterThisMonth => "after_this_month",
            SearchQueryConditionOperator::ThisQuarter => "this_quarter",
            SearchQueryConditionOperator::BeforeThisQuarter => "before_this_quarter",
            SearchQueryConditionOperator::AfterThisQuarter => "after_this_quarter",
            SearchQueryConditionOperator::ThisYear => "this_year",
            SearchQueryConditionOperator::BeforeThisYear => "before_this_year",
            SearchQueryConditionOperator::AfterThisYear => "after_this_year",
            SearchQueryConditionOperator::AnyOf => "any_of",
            SearchQueryConditionOperator::NoneOf => "none_of",
            SearchQueryConditionOperator::IsTrue => "is_true",
            SearchQueryConditionOperator::IsFalse => "is_false",
            SearchQueryConditionOperator::Address => "address",
        };
        expr.push_str(operator);

        let (name, value) = match hashmap.get(&condition.object_attribute_id) {
            Some(oa) => {
                let name = oa.attributes.name.clone();
                let mut value = condition.value;

                if oa.attributes.data_type == "picklist" {
                    value = match value {
                        Some(value) => Some(process_picklist_oa_value(oa, value)),
                        None => None,
                    };
                }

                (name, value)
            }
            None => ("not found".to_string(), condition.value),
        };

        let str = match value {
            Some(value) => format!(" {name} {value}"),
            None => format!(" {name}"),
        };

        expr.push_str(&str);

        expr.push(')');
    }

    expr.push(')');

    expr
}

fn parse_search_query(
    search_query: SearchQuery,
    hashmap: &HashMap<String, ObjectAttribute>,
) -> Result<String> {
    let mut expr = "".to_owned();

    for group in search_query.search_query_groups {
        let str = parse_search_query_group(group, hashmap);
        expr.push_str(&str);
    }

    Ok(expr)
}

pub fn parse_payload(
    file_path: String,
    hashmap: &HashMap<String, ObjectAttribute>,
) -> Result<HashMap<String, Value>> {
    let file = File::open(file_path).expect("failed to open payload");
    let reader = BufReader::new(file);
    let payload: Payload = serde_json::from_reader(reader).expect("failed to parse payload file");

    let mut map = HashMap::new();

    let names = payload
        .object_attribute_ids
        .into_iter()
        .map(|id| match hashmap.get(&id) {
            Some(oa) => oa.attributes.name.clone(),
            None => "not found".to_string(),
        })
        .collect();

    map.insert("names".to_string(), names);

    let search_query = parse_search_query(payload.search_query, hashmap)?;

    map.insert(
        "search_query".to_string(),
        serde_json::Value::String(search_query),
    );

    Ok(map)
}
