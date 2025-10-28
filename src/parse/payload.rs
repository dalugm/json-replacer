use anyhow::Result;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

use super::{
    ObjectAttribute, ObjectAttributeDataType, SearchQuery, SearchQueryCondition,
    SearchQueryConditionOperator, SearchQueryGroup, SearchQueryGroupOperator, convert_raw_entity,
};

#[derive(Deserialize)]
pub struct Payload {
    object_attribute_ids: Option<Vec<String>>,
    search_query: Option<SearchQuery>,
    pub object_entity_attribute_values: Option<HashMap<String, Value>>,
}

/// Transform picklist oa id to name.
fn process_picklist_oa_value(oa: &ObjectAttribute, value: Value) -> Value {
    match value {
        Value::String(option_id) => {
            let picklist_option = oa
                .picklist_options
                .iter()
                .find(|option| option.id == option_id);

            match picklist_option {
                Some(option) => serde_json::Value::String(option.name.clone()),
                None => {
                    println!("Picklist option not found for id: {option_id}");
                    "not_found_picklist_label".into()
                }
            }
        }
        Value::Array(option_ids) => option_ids
            .iter()
            .map(|option_id| {
                let picklist_option = oa
                    .picklist_options
                    .iter()
                    .find(|option| option.id == *option_id);

                match picklist_option {
                    Some(option) => serde_json::Value::String(option.name.clone()),
                    None => {
                        println!("Picklist option not found for id: {option_id}");
                        "not_found_picklist_label".into()
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
                    let name = oa.name.clone();
                    let mut value = condition.value;

                    if oa.data_type == ObjectAttributeDataType::Picklist {
                        value = value.map(|value| process_picklist_oa_value(oa, value));
                    }

                    (name, value)
                }
                None => ("not_found".to_string(), condition.value),
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

pub fn parse(
    payload: Payload,
    hashmap: &HashMap<String, ObjectAttribute>,
) -> Result<HashMap<String, Value>> {
    let mut map = HashMap::new();

    if let Some(ids) = payload.object_attribute_ids {
        let search_oa_list = ids
            .into_iter()
            .map(|id| {
                let name = match hashmap.get(&id) {
                    Some(oa) => oa.name.clone(),
                    None => "not_found".to_string(),
                };

                format!("{name}, {id}")
            })
            .collect::<Vec<String>>();

        map.insert("object_attributes".to_string(), search_oa_list.into());
    }

    if let Some(search_query) = payload.search_query {
        let search_query = parse_search_query(search_query, hashmap)?;

        map.insert(
            "search_query".to_string(),
            serde_json::Value::String(search_query),
        );
    }

    if let Some(entity) = payload.object_entity_attribute_values {
        let object_entity = convert_raw_entity(entity, hashmap);

        let serde_object = serde_json::to_value(object_entity)?;

        map.insert("object_entity_attribute_values".to_string(), serde_object);
    }

    Ok(map)
}
