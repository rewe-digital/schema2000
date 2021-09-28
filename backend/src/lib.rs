use serde_json::Value;

#[derive(Debug)]
pub struct SchemaHypothesis {}

pub fn generate_hypothesis(_dom: &Value) -> SchemaHypothesis {
    SchemaHypothesis {}
}

pub fn merge_hypothesis(_a: SchemaHypothesis, _b: SchemaHypothesis) -> SchemaHypothesis {
    SchemaHypothesis {}
}
