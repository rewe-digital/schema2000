#![allow(clippy::module_name_repetitions)]

use serde_json::Value;
use std::collections::HashMap;

pub use generate::generate_hypothesis;
pub use merge::merge_hypothesis;
pub use model::SchemaHypothesis;
pub use renderer::render_schema;

mod generate;
mod merge;
pub mod model;
mod renderer;
mod utils;

pub fn generate_hypothesis_from_jsons(
    json_documents: Vec<serde_json::Result<Value>>,
) -> Result<HashMap<String, SchemaHypothesis>, Box<dyn std::error::Error>> {
    let mut hypothesises: HashMap<String, SchemaHypothesis> = HashMap::new();

    for json_document in json_documents {
        let document = &json_document?;
        let discriminator_key = &extract_discriminator(document);
        let new_hypo = generate_hypothesis(extract_payload(document));

        if hypothesises.contains_key(discriminator_key) {
            let current = hypothesises.get(discriminator_key).unwrap().clone();
            let merged_hypo = merge_hypothesis(current, new_hypo);
            hypothesises.insert(discriminator_key.to_string(), merged_hypo);
        } else {
            hypothesises.insert(discriminator_key.to_string(), new_hypo);
        }
    }
    Ok(hypothesises)
}

fn extract_payload(document: &Value) -> &Value {
    document.as_object().unwrap().get("payload").unwrap()
}

fn extract_discriminator(document: &Value) -> String {
    document
        .as_object()
        .unwrap()
        .get("discriminator")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string()
}
