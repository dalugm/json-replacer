mod common;

use std::{collections::HashMap, fs, path::Path};

use serde_json::Value;

use json_replacer::{Payload, Response, process_object_entity, process_payload, process_response};

fn get_content(input: String) -> String {
    let trimmed_input = input.trim();

    // Treat as json str.
    if trimmed_input.starts_with('{') || trimmed_input.starts_with('[') {
        return input;
    }

    let path = Path::new(&input);
    if path.exists() {
        // Treat as file.
        return fs::read_to_string(path).unwrap();
    }

    input
}

#[test]
fn parse_reference() {
    let reference = common::setup();

    assert!(reference.contains_key("019883f0-c110-7bc5-854e-26a7135a9ec0"));
}

#[test]
fn parse_payload() {
    let reference = common::setup();

    let payload_content = get_content("tests/payload.json".to_string());
    let payload = serde_json::from_str::<Payload>(&payload_content).unwrap();

    let payload = process_payload(&reference, payload).expect("failed to parse payload");

    let object_attributes = payload.get("object_attributes").unwrap();

    assert_eq!(
        object_attributes,
        &serde_json::Value::Array(vec![serde_json::Value::String(
            "Type_Name, 019883f0-c110-7bc5-854e-26a7135a9ec0".to_string()
        )])
    );

    let search_query = payload.get("search_query").unwrap();

    assert_eq!(
        search_query,
        &serde_json::Value::String(
            "(AND (equal Type_Name \"not_found_picklist_label\"))".to_string()
        )
    );
}

#[test]
fn parse_response() {
    let reference = common::setup();

    let response_content = get_content("tests/response.json".to_string());
    let response = serde_json::from_str::<Response>(&response_content).unwrap();

    let response = process_response(&reference, response).expect("failed to parse resposne");

    assert_eq!(response.len(), 2);
}

#[test]
fn parse_object_entity() {
    let reference = common::setup();

    let object_entity_content = get_content("tests/object_entity.json".to_string());
    let object_entity =
        serde_json::from_str::<HashMap<String, Value>>(&object_entity_content).unwrap();

    let entity =
        process_object_entity(&reference, object_entity).expect("failed to parse object entity");

    assert_eq!(entity.len(), 3);
}
