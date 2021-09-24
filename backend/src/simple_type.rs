use maplit::hashset;
use serde_json::Value;
use std::collections::HashSet;

#[derive(Debug)]
pub enum SimpleType {
    Array,
    Boolean,
    Integer,
    IntegerEnum(HashSet<i64>),
    Null,
    Number,
    Object,
    String,
    StringEnum(HashSet<String>),
}

impl SimpleType {
    pub fn new(value: &Value) -> Self {
        match value {
            Value::Null => SimpleType::Null,
            Value::Bool(_) => SimpleType::Boolean,
            Value::Number(n) if n.is_i64() => {
                SimpleType::IntegerEnum(hashset! {n.as_i64().unwrap()})
            }
            Value::Number(_) => SimpleType::Number,
            Value::String(s) => SimpleType::StringEnum(hashset! {s.clone()}),
            Value::Array(_a) => SimpleType::Array, // this would need to recurse down the array and all SimpleType::new(_a)
            Value::Object(_) => SimpleType::Object, // same as for the array: walk along all elements
        }
    }
}
