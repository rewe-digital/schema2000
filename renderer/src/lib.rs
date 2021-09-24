use backend::SchemaBuilder;

pub fn render_debug(sb: &SchemaBuilder) -> String {
    format!("{:#?}", sb.root)
}

pub fn render_schema(_sb: &SchemaBuilder) -> String {
    todo!()
}

pub fn render_typescript(_sb: &SchemaBuilder) -> String {
    todo!()
}

pub fn render_haskell(_sb: &SchemaBuilder) -> String {
    todo!()
}

pub fn render_kotlin(_sb: &SchemaBuilder) -> String {
    todo!()
}
