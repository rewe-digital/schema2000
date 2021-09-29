mod json_schema_renderer;
mod utils;

use backend::SchemaHypothesis;

pub use json_schema_renderer::render_schema;

pub fn render_debug(schema: &SchemaHypothesis) -> String {
    format!("{:#?}", schema)
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
