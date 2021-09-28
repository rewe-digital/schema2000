use backend::SchemaHypothesis;

pub fn render_debug(schema: &SchemaHypothesis) -> String {
    format!("{:#?}", schema)
}

pub fn render_schema(_schema: &SchemaHypothesis) -> String {
    todo!()
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
