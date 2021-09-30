use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Map, Value};

use crate::utils::SetVariances;
use crate::{NodeType, ObjectProperty, SchemaHypothesis};

fn generate_properties(properties: &Map<String, Value>) -> BTreeMap<String, ObjectProperty> {
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
        Value::Array(array_values) => {
            if array_values.is_empty() {
                NodeType::new_untyped_array()
            } else {
                NodeType::Array(Some(Box::new(generate_node_type_for_array_values(
                    array_values,
                ))))
            }
        }
        Value::Object(props) => NodeType::Object {
            properties: generate_properties(props),
        },
    }
}

fn generate_node_type_for_array_values(array_values: &[Value]) -> NodeType {
    let mut merged_obj_type: Option<NodeType> = None;
    let mut types = BTreeSet::new();

    for value in array_values.iter() {
        let value_type = generate_node_type(value);
        match value_type {
            NodeType::Object { properties: _ } => {
                merged_obj_type = match merged_obj_type {
                    Some(acc) => Some(crate::merge::merge_node_type(acc, value_type)),
                    None => Some(value_type),
                };
            }
            _ => {
                types.insert(value_type);
            }
        };
    }
    if let Some(node_type) = merged_obj_type {
        types.insert(node_type);
    }

    match SetVariances::new(&types) {
        SetVariances::Empty => unreachable!(),
        SetVariances::OneElement(node_type) => node_type.clone(),
        SetVariances::Multiple(_) => NodeType::Any(types),
    }
}

#[must_use]
pub fn generate_hypothesis(dom: &Value) -> SchemaHypothesis {
    #![allow(clippy::module_name_repetitions)]
    SchemaHypothesis {
        root: generate_node_type(dom),
    }
}

#[cfg(test)]
mod test {
    use maplit::{btreemap, btreeset};
    use serde_json::json;

    use crate::generate::generate_node_type;
    use crate::{NodeType, ObjectProperty};

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

    #[test]
    fn test_array_merge_objects() {
        let dom = json!(["one", 1, {"a": 1}, {"a": "1"}]);
        let actual = generate_node_type(&dom);
        let expected = NodeType::new_typed_array(btreeset! {
            NodeType::String,
            NodeType::Integer,
            NodeType::Object {
                properties: btreemap! {
                    "a".to_string() => ObjectProperty { required: true, node_type: NodeType::Any(
                        btreeset! { NodeType::String, NodeType::Integer }
                    )}
                }
            }
        });
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_array_all_int() {
        let dom = json!([10, 15, 25]);
        assert_eq!(
            generate_node_type(&dom),
            NodeType::Array(Some(Box::new(NodeType::Integer)))
        );
    }

    #[test]
    fn test_array_empty() {
        let dom = json!([]);
        assert_eq!(generate_node_type(&dom), NodeType::new_untyped_array());
    }

    #[test]
    fn test_array_int_and_string() {
        let dom = json!([42, "Hello"]);
        assert_eq!(
            generate_node_type(&dom),
            NodeType::new_typed_array(btreeset![NodeType::Integer, NodeType::String])
        );
    }

    #[test]
    fn test_object() {
        let dom = json!({
            "name": "Schokoladenbrunnen",
            "length": 100
        });
        assert_eq!(
            generate_node_type(&dom),
            NodeType::Object {
                properties: btreemap! {
                    "name".to_string() => ObjectProperty{ required: true, node_type: NodeType::String },
                    "length".to_string() => ObjectProperty{ required: true, node_type: NodeType::Integer }
                }
            }
        );
    }
}
