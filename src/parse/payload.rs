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
    let mut lisp_expr_vec: Vec<String> = Vec::new();

    let operator = match group.operator {
        SearchQueryGroupOperator::And => "AND",
        SearchQueryGroupOperator::Or => "OR",
        SearchQueryGroupOperator::Not => "NOT",
    };

    lisp_expr_vec.push(operator.to_string());

    if let Some(conditions) = group.search_query_conditions {
        let cond_expr = parse_search_query_conditions(conditions, hashmap);
        lisp_expr_vec.push(cond_expr);
    }

    if let Some(children) = group.children {
        let children_expr = parse_search_query_children(children, hashmap);
        lisp_expr_vec.push(children_expr);
    }

    let lisp_expr = format!("({})", lisp_expr_vec.join(" "));

    lisp_expr
}

fn parse_search_query_children(
    children: Vec<SearchQueryGroup>,
    hashmap: &HashMap<String, ObjectAttribute>,
) -> String {
    children
        .into_iter()
        .map(|child| parse_search_query_group(child, hashmap))
        .collect::<Vec<String>>()
        .join(" ")
}

fn parse_search_query_conditions(
    conditions: Vec<SearchQueryCondition>,
    hashmap: &HashMap<String, ObjectAttribute>,
) -> String {
    let lisp_exprs: Vec<String> = conditions
        .into_iter()
        .map(|condition| {
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

            let (name, value) = match hashmap.get(&condition.object_attribute_id) {
                Some(oa) => {
                    let name = oa.attributes.name.clone();
                    let mut value = condition.value;

                    if oa.attributes.data_type == "picklist" {
                        value = value.map(|value| process_picklist_oa_value(oa, value));
                    }

                    (name, value)
                }
                None => ("not found".to_string(), condition.value),
            };

            let args = match value {
                Some(value) => &format!("{name} {value}"),
                None => &name,
            };

            let lisp_expr = format!("({})", [operator, args].join(" "));

            lisp_expr
        })
        .collect();

    lisp_exprs.join(" ").to_string()
}

fn parse_search_query(
    search_query: SearchQuery,
    hashmap: &HashMap<String, ObjectAttribute>,
) -> Result<String> {
    let lisp_expr = search_query
        .search_query_groups
        .into_iter()
        .map(|group| parse_search_query_group(group, hashmap))
        .collect::<Vec<String>>()
        .join(" ");

    Ok(lisp_expr)
}

pub fn parse_payload(
    file_path: String,
    hashmap: &HashMap<String, ObjectAttribute>,
) -> Result<HashMap<String, Value>> {
    let file = File::open(file_path).expect("failed to open payload");
    let reader = BufReader::new(file);
    let payload: Payload = serde_json::from_reader(reader).expect("failed to parse payload file");

    let mut map = HashMap::new();

    let search_oa_list = payload
        .object_attribute_ids
        .into_iter()
        .map(|id| {
            let name = match hashmap.get(&id) {
                Some(oa) => oa.attributes.name.clone(),
                None => "not found".to_string(),
            };

            format!("{name}, {id}")
        })
        .collect();

    map.insert("object_attributes".to_string(), search_oa_list);

    let search_query = parse_search_query(payload.search_query, hashmap)?;

    map.insert(
        "search_query".to_string(),
        serde_json::Value::String(search_query),
    );

    Ok(map)
}
