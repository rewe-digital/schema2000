use std::collections::{BTreeMap, BTreeSet};

use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;

use backend::{NodeType, ObjectProperty, SchemaHypothesis};

#[must_use]
pub fn render_schema(schema: &SchemaHypothesis) -> String {
    serde_json::to_string_pretty(&render_json_schema(schema)).unwrap()
}

fn render_json_schema(schema: &SchemaHypothesis) -> Value {
    render_node(&schema.root)
}

fn render_node(node_type: &NodeType) -> Value {
    match node_type {
        NodeType::String => json!({"type": "string"}),
        NodeType::Integer => json!({"type": "integer"}),
        NodeType::Number => json!({"type": "number"}),
        NodeType::Boolean => json!({"type": "boolean"}),
        NodeType::Null => json!({"type": "null"}),
        NodeType::Array(node_types) => Value::Object(generate_array_map(node_types)),
        NodeType::Object { properties } => Value::Object(generate_object_map(properties)),
        NodeType::Any(node_types) => Value::Object(generate_any_map(node_types)),
    }
}

fn generate_any_map(node_types: &BTreeSet<NodeType>) -> Map<String, Value> {
    let mut map = Map::new();
    map.insert(
        "anyOf".to_string(),
        node_types.iter().map(render_node).collect(),
    );

    map
}

fn generate_array_map(node_type: &Option<Box<NodeType>>) -> Map<String, Value> {
    let mut map = Map::new();
    map.insert("type".to_string(), Value::String("array".to_string()));
    node_type
        .as_ref()
        .map(|node_type| map.insert("items".to_string(), render_node(node_type)));
    map
}

fn generate_object_map(properties: &BTreeMap<String, ObjectProperty>) -> Map<String, Value> {
    let required_props: Vec<Value> = properties
        .iter()
        .filter_map(|(key, value)| {
            if value.required {
                Option::Some(Value::String(key.to_string()))
            } else {
                Option::None
            }
        })
        .collect();

    let object_properties: Map<String, Value> = properties
        .iter()
        .map(|(key, value)| (key.to_string(), render_node(&value.node_type)))
        .collect();

    let mut map = Map::new();

    map.insert("type".to_string(), Value::String("object".to_string()));
    map.insert("required".to_string(), Value::Array(required_props));
    map.insert("properties".to_string(), Value::Object(object_properties));

    map
}

#[cfg(test)]
mod test {
    use maplit::{btreemap, btreeset};
    use serde_json::json;
    use std::collections::BTreeSet;

    use backend::{NodeType, ObjectProperty, SchemaHypothesis};

    use crate::json_schema_renderer::{render_json_schema, render_node};

    #[test]
    fn test_object() {
        let hypothesis = SchemaHypothesis {
            root: NodeType::Object {
                properties: btreemap! {
                    "name".to_string() => ObjectProperty{ required: true, node_type: NodeType::String },
                },
            },
        };

        let actual = render_json_schema(&hypothesis);

        assert_eq!(
            actual,
            json!(
                {
                    "type": "object",
                    "required": ["name"],
                    "properties": {
                        "name": {
                            "type": "string"
                        }
                    }
                }
            )
        );
    }

    #[test]
    fn test_array() {
        let hypothesis = SchemaHypothesis {
            root: NodeType::new_typed_array(btreeset![NodeType::String, NodeType::Integer]),
        };

        let actual = render_json_schema(&hypothesis);

        assert_eq!(
            actual,
            json!(
                {
                    "type": "array",
                    "items": {
                        "anyOf": [
                            {
                                "type": "string"
                            },
                            {
                                "type": "integer"
                            }
                        ]
                    }
                }
            )
        );
    }

    #[test]
    fn test_array_single_type() {
        let hypothesis = SchemaHypothesis {
            root: NodeType::new_typed_array(btreeset!(NodeType::String)),
        };

        let actual = render_json_schema(&hypothesis);

        assert_eq!(
            actual,
            json!(
                {
                    "type": "array",
                    "items": {
                        "type": "string"
                    }
                }
            )
        );
    }

    #[test]
    fn test_empty_array() {
        let hypothesis = SchemaHypothesis {
            root: NodeType::new_untyped_array(),
        };

        let actual = render_json_schema(&hypothesis);

        assert_eq!(actual, json!({ "type": "array" }));
    }

    #[test]
    fn test_any() {
        let node_type = NodeType::Any(btreeset![NodeType::String, NodeType::Boolean]);

        let actual = render_node(&node_type);

        assert_eq!(
            actual,
            json!({
                "anyOf": [
                    {"type": "string"},
                    {"type": "boolean"},
                ]
            })
        );
    }

    #[test]
    fn test_any_one() {
        let node_type = NodeType::Any(btreeset![NodeType::String]);

        let actual = render_node(&node_type);

        assert_eq!(
            actual,
            json!({
                "anyOf": [
                    {"type": "string"}
                ]
            })
        );
    }

    #[test]
    fn test_any_empty() {
        let node_type = NodeType::Any(btreeset![]);

        let actual = render_node(&node_type);

        assert_eq!(
            actual,
            json!({
                "anyOf": []
            })
        );
    }

    #[test]
    fn test_any_complex_types() {
        let node_type = NodeType::Any(btreeset![NodeType::Object {
            properties: btreemap! {
                "id".to_string() => ObjectProperty {
                    node_type: NodeType::Integer,
                    required: true
                }
            }
        }]);

        let actual = render_node(&node_type);

        assert_eq!(
            actual,
            json!({
                "anyOf": [
                    {
                        "type": "object",
                        "properties": {
                            "id": {
                                "type": "integer"
                            }
                        },
                        "required": ["id"]
                    }
                ]
            })
        );
    }
}
