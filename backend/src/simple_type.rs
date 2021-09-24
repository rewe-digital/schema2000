use crate::integer_type::IntegerType;
use crate::string_type::StringType;
use serde_json::Value;

// TODO: what about enum?
#[derive(Debug)]
pub enum SimpleType {
    Array,
    Boolean,
    // maybe we should inline IntegerType, so we have SimpleType::Integer and SimpleType::IntegerEnum
    Integer(IntegerType),
    Null,
    Number,
    Object,
    // same as for integer
    String(StringType),
}

impl SimpleType {
    pub fn new(value: &Value) -> Self {
        match value {
            Value::Null => SimpleType::Null,
            Value::Bool(_) => SimpleType::Boolean,
            Value::Number(n) if n.is_i64() => {
                SimpleType::Integer(IntegerType::new(n.as_i64().unwrap()))
            }
            Value::Number(_) => SimpleType::Number,
            Value::String(s) => SimpleType::String(StringType::new(s)),
            Value::Array(_a) => SimpleType::Array, // this would need to recurse down the array and all SimpleType::new(_a)
            Value::Object(_) => SimpleType::Object, // same as for the array: walk along all elements
        }
    }
}
