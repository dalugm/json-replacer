use json_replacer::parse::{
    object_entity::parse as eparse, payload::parse as pparse, response::parse as rparse,
};

mod common;

#[test]
fn parse_reference() {
    let reference = common::setup();

    assert!(reference.contains_key("019883f0-c110-7bc5-854e-26a7135a9ec0"));
}

#[test]
fn parse_payload() {
    let reference = common::setup();

    let payload =
        pparse("tests/payload.json".to_string(), &reference).expect("failed to parse payload");

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

    let response =
        rparse("tests/response.json".to_string(), &reference).expect("failed to parse resposne");

    assert_eq!(response.len(), 2);
}

#[test]
fn parse_object_entity() {
    let reference = common::setup();

    let entity = eparse("tests/object_entity.json".to_string(), &reference)
        .expect("failed to parse object entity");

    assert_eq!(entity.len(), 3);
}
