mod json_schema_renderer;

use crate::json_schema_renderer::render_json_schema;
use backend::SchemaHypothesis;
use serde_json::to_string_pretty;

pub fn render_debug(schema: &SchemaHypothesis) -> String {
    format!("{:#?}", schema)
}

pub fn render_schema(schema: &SchemaHypothesis) -> String {
    to_string_pretty(&render_json_schema(schema)).unwrap()
}

pub fn render_typescript(_schema: &SchemaHypothesis) -> String {
    todo!()
}

pub fn render_haskell(_schema: &SchemaHypothesis) -> String {
    todo!()
}

pub fn render_kotlin(_schema: &SchemaHypothesis) -> String {
    todo!()
}
