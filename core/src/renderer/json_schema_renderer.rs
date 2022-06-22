use std::collections::{BTreeMap, BTreeSet};

use serde_json::value::Value;
use serde_json::Map;
use serde_json::{json, Number};

use crate::model::{ArrayNode, IntegerNode, NodeType, ObjectNode, ObjectProperty, StringNode};
use crate::SchemaHypothesis;

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn render_schema(schema: &SchemaHypothesis) -> String {
    serde_json::to_string_pretty(&render_json_schema(schema)).unwrap()
}

fn render_json_schema(schema: &SchemaHypothesis) -> Value {
    render_node(&schema.root)
}

fn render_node(node_type: &NodeType) -> Value {
    match node_type {
        NodeType::String(node) => Value::Object(generate_string(node)),
        NodeType::Integer(node) => Value::Object(generate_integer(node)),
        NodeType::Number(_) => json!({"type": "number"}),
        NodeType::Boolean => json!({"type": "boolean"}),
        NodeType::Null => json!({"type": "null"}),
        NodeType::Array(node_types) => Value::Object(generate_array_map(node_types)),
        NodeType::Object(ObjectNode { properties }) => {
            Value::Object(generate_object_map(properties))
        }
        NodeType::Any(node_types) => Value::Object(generate_any_map(&node_types.nodes)),
    }
}

fn generate_integer(node: &IntegerNode) -> Map<String, Value> {
    generate_enumerable("integer", node.values.values(), |v| {
        Value::Number(Number::from(*v))
    })
}

fn generate_string(node: &StringNode) -> Map<String, Value> {
    generate_enumerable("string", node.values.values(), |v: &String| {
        // TODO what to do instead of v.to_string to get a &str
        Value::String(v.to_string())
    })
}

fn generate_enumerable<T>(
    type_as_string: &str,
    values: Option<Vec<&T>>,
    serde_value_supplier: fn(&T) -> Value,
) -> Map<String, Value> {
    let mut map = Map::new();
    map.insert(
        "type".to_string(),
        Value::String(type_as_string.to_string()),
    );
    if let Some(vs) = values {
        if !vs.is_empty() {
            map.insert(
                "enum".to_string(),
                Value::Array(vs.into_iter().map(serde_value_supplier).collect()),
            );
        }
    }
    map
}

fn generate_any_map(node_types: &BTreeSet<NodeType>) -> Map<String, Value> {
    let mut map = Map::new();
    map.insert(
        "anyOf".to_string(),
        node_types.iter().map(render_node).collect(),
    );

    map
}

fn generate_array_map(node_type: &ArrayNode) -> Map<String, Value> {
    let mut map = Map::new();
    map.insert("type".to_string(), Value::String("array".to_string()));
    node_type
        .items
        .as_ref()
        .map(|node_type| map.insert("items".to_string(), render_node(node_type)));
    map
}

fn generate_object_map(properties: &BTreeMap<String, ObjectProperty>) -> Map<String, Value> {
    let required_props: Vec<Value> = properties
        .iter()
        .filter_map(|(key, value)| {
            if value.required {
                Some(Value::String(key.to_string()))
            } else {
                None
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

    use crate::model::{
        AnyNode, ArrayNode, IntegerNode, NodeType, ObjectNode, ObjectProperty, SchemaHypothesis,
        StringNode, ValueCollection,
    };
    use crate::renderer::json_schema_renderer::{render_json_schema, render_node};

    #[test]
    fn test_string_without_values() {
        let hypothesis = SchemaHypothesis::new(StringNode {
            values: ValueCollection::empty_collection(),
        });

        let actual = render_json_schema(&hypothesis);

        assert_eq!(
            actual,
            json!(
                {
                    "type": "string"
                }
            )
        );
    }

    #[test]
    fn test_string_with_values() {
        let hypothesis = SchemaHypothesis::new(StringNode::with_values(vec!["a", "b", "c"]));

        let actual = render_json_schema(&hypothesis);

        assert_eq!(
            actual,
            json!(
                {
                    "type": "string",
                    "enum": ["a", "b", "c"]
                }
            )
        );
    }

    #[test]
    fn test_object() {
        let hypothesis = SchemaHypothesis::new(ObjectNode::new(btreemap! {
            "name".to_string() => ObjectProperty::new(StringNode::new()),
        }));

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
        let hypothesis = SchemaHypothesis::new(ArrayNode::new_many(btreeset![
            StringNode::new().into(),
            IntegerNode::new().into()
        ]));

        let actual = render_json_schema(&hypothesis);

        assert_eq!(
            actual,
            json!(
                {
                    "type": "array",
                    "items": {
                        "anyOf": [
                            {
                                "type": "integer"
                            },
                            {
                                "type": "string"
                            }
                        ]
                    }
                }
            )
        );
    }

    #[test]
    fn test_array_single_type() {
        let hypothesis =
            SchemaHypothesis::new(ArrayNode::new_many(btreeset!(StringNode::new().into())));

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
        let hypothesis = SchemaHypothesis::new(ArrayNode::new_untyped());

        let actual = render_json_schema(&hypothesis);

        assert_eq!(actual, json!({ "type": "array" }));
    }

    #[test]
    fn test_any() {
        let node_type = AnyNode::new(btreeset![StringNode::new().into(), NodeType::Boolean]).into();

        let actual = render_node(&node_type);

        assert_eq!(
            actual,
            json!({
                "anyOf": [
                    {"type": "boolean"},
                    {"type": "string"},
                ]
            })
        );
    }

    #[test]
    fn test_any_one() {
        let node_type = AnyNode::new(btreeset![StringNode::new().into()]).into();

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
        let node_type = AnyNode::new(btreeset![]).into();

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
        let node_type = AnyNode::new(btreeset![ObjectNode::new(btreemap! {
            "id".to_string() => ObjectProperty::new(IntegerNode::new())
        })
        .into()])
        .into();

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
