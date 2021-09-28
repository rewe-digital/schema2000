pub use merge::merge_hypothesis;
use serde_json::{Map, Value};
use std::collections::HashMap;

mod merge;

#[derive(Debug, PartialEq)]
pub struct SchemaHypothesis {
    root: NodeType,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectProperty {
    node_type: NodeType,
    required: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NodeType {
    String,
    Integer,
    Number,
    Boolean,
    Null,
    Array(Box<NodeType>),
    Object {
        properties: HashMap<String, ObjectProperty>,
    },
    // TODO Use a HashSet. Problem: Neither HashSet nor HashMap implement the Hash trait
    Any(Vec<NodeType>),
}

fn generate_properties(properties: &Map<String, Value>) -> HashMap<String, ObjectProperty> {
    properties
        .iter()
        .map(|(key, value)| {
            (
                key.clone(),
                ObjectProperty {
                    required: true,
                    node_type: generate_node_type(value),
                },
            )
        })
        .collect()
}

fn generate_node_type(dom: &Value) -> NodeType {
    match dom {
        Value::Null => NodeType::Null,
        Value::Bool(_) => NodeType::Boolean,
        Value::Number(i) if i.is_f64() => NodeType::Number,
        Value::Number(_) => NodeType::Integer,
        Value::String(_) => NodeType::String,
        // TODO: not done yet
        Value::Array(_) => NodeType::Array(Box::new(NodeType::Null)),
        Value::Object(props) => NodeType::Object {
            properties: generate_properties(props),
        },
    }
}

#[must_use]
pub fn generate_hypothesis(dom: &Value) -> SchemaHypothesis {
    SchemaHypothesis {
        root: generate_node_type(dom),
    }
}

#[cfg(test)]
mod test {
    use crate::{generate_node_type, NodeType, ObjectProperty};
    use maplit::hashmap;
    use serde_json::json;

    #[test]
    fn test_null() {
        let dom = json!(null);
        assert_eq!(generate_node_type(&dom), NodeType::Null);
    }

    #[test]
    fn test_bool() {
        let dom = json!(true);
        assert_eq!(generate_node_type(&dom), NodeType::Boolean);
    }

    #[test]
    fn test_integer() {
        let dom = json!(10);
        assert_eq!(generate_node_type(&dom), NodeType::Integer);
    }

    #[test]
    fn test_number() {
        let dom = json!(10.5);
        assert_eq!(generate_node_type(&dom), NodeType::Number);
    }

    #[test]
    fn test_string() {
        let dom = json!("Schema 2000");
        assert_eq!(generate_node_type(&dom), NodeType::String);
    }

    // #[test]
    // fn test_array() {
    //     let dom = json!([10, 15, 25]);
    //     assert_eq!(generate_node_type(&dom), NodeType::Array(Box::new(NodeType::Integer)));
    // }

    #[test]
    fn test_object() {
        let dom = json!({
            "name": "Schokoladenbrunnen",
            "length": 100
        });
        assert_eq!(
            generate_node_type(&dom),
            NodeType::Object {
                properties: hashmap! {
                    "name".to_string() => ObjectProperty{ required: true, node_type: NodeType::String },
                    "length".to_string() => ObjectProperty{ required: true, node_type: NodeType::Integer }
                }
            }
        );
    }
}
