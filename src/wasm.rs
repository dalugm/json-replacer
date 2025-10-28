use std::collections::HashMap;

use serde_json::Value;
use wasm_bindgen::prelude::*;

use super::{
    ObjectAttribute, Payload, Response, parse::reference::ObjectAttributesRaw,
    preprocess_reference, process_object_entity, process_payload, process_response,
};

#[wasm_bindgen]
pub struct Processor {
    hashmap: HashMap<String, ObjectAttribute>,
}

#[wasm_bindgen]
impl Processor {
    #[wasm_bindgen(constructor)]
    pub fn new(reference: JsValue) -> Result<Processor, String> {
        let reference_struct: ObjectAttributesRaw =
            serde_wasm_bindgen::from_value(reference).map_err(|e| e.to_string())?;

        let hashmap = preprocess_reference(reference_struct).map_err(|e| e.to_string())?;

        Ok(Processor { hashmap })
    }

    #[wasm_bindgen]
    pub fn payload(&self, payload: JsValue) -> Result<JsValue, JsValue> {
        let payload: Payload = serde_wasm_bindgen::from_value(payload)?;
        let result = process_payload(&self.hashmap, payload).map_err(|e| e.to_string())?;
        serde_wasm_bindgen::to_value(&result).map_err(|e| e.to_string().into())
    }

    #[wasm_bindgen]
    pub fn response(&self, response: JsValue) -> Result<JsValue, JsValue> {
        let response: Response = serde_wasm_bindgen::from_value(response)?;
        let result = process_response(&self.hashmap, response).map_err(|e| e.to_string())?;
        serde_wasm_bindgen::to_value(&result).map_err(|e| e.to_string().into())
    }

    #[wasm_bindgen]
    pub fn entity(&self, object_entity: JsValue) -> Result<JsValue, JsValue> {
        let entity: HashMap<String, Value> = serde_wasm_bindgen::from_value(object_entity)?;
        let result = process_object_entity(&self.hashmap, entity).map_err(|e| e.to_string())?;
        serde_wasm_bindgen::to_value(&result).map_err(|e| e.to_string().into())
    }
}
