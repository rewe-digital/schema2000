use serde_json::Value;

// TODO: what about enum?
#[derive(Debug)]
pub enum SimpleType {
    Array,
    Boolean,
    Integer,
    Null,
    Number,
    Object,
    String,
}

impl SimpleType {
    pub fn new(value: &Value) -> Self {
        match value {
            Value::Null => SimpleType::Null,
            Value::Bool(_) => SimpleType::Boolean,
            Value::Number(_) => SimpleType::Number,
            Value::String(_) => SimpleType::String,
            Value::Array(_a) => SimpleType::Array, // this would need to recurse down the array and all SimpleType::new(_a)
            Value::Object(_) => SimpleType::Object, // same as for the array: walk along all elements
        }
    }
}
