use std::collections::HashMap;

use serde_json::json;
use serde_json::Map;
use serde_json::value::Value;

use backend::{NodeType, ObjectProperty, SchemaHypothesis};

pub fn render_json_schema(schema: &SchemaHypothesis) -> Value {
    render_node(&schema.root)
}

fn render_node(node_type: &NodeType) -> Value {
    match node_type {
        NodeType::String => json!({"type": "string"}),
        NodeType::Integer => json!({"type": "integer"}),
        NodeType::Number => json!({"type": "number"}),
        NodeType::Boolean => json!({"type": "boolean"}),
        NodeType::Null => json!({"type": "null"}),
        NodeType::Array(_) => unimplemented!(),
        NodeType::Object { properties } => {
            Value::Object(generate_object_map(properties))
        }
        NodeType::Any(_) => unimplemented!()
    }
}

fn generate_object_map(properties: &HashMap<String, ObjectProperty>) -> Map<String, Value> {
    let required_props: Vec<Value> = properties.iter().flat_map(|(key, value)|
        if value.required {
            Option::Some(Value::String(key.to_string()))
        } else {
            Option::None
        }
    ).collect();

    let object_properties: Map<String, Value> = properties.iter().map(|(key, value)|
        (key.to_string(), render_node(&value.node_type))
    ).collect();

    let mut map = Map::new();

    map.insert("type".to_string(),Value::String(String::from("object")));
    map.insert("required".to_string(),Value::Array(required_props));
    map.insert("properties".to_string(),Value::Object(object_properties));

    map
}

#[cfg(test)]
mod test {
    use maplit::hashmap;
    use serde_json::json;

    use backend::{NodeType, ObjectProperty, SchemaHypothesis};

    use crate::json_schema_renderer::render_json_schema;

    #[test]
    fn test_object() {
        let hypothesis = SchemaHypothesis {
            root: NodeType::Object {
                properties: hashmap! {
                    "name".to_string() => ObjectProperty{ required: true, node_type: NodeType::String },
                }
            }
        };

        let actual = render_json_schema(&hypothesis);

        assert_eq!(actual, json!(
            {
                "type": "object",
                "required": ["name"],
                "properties": {
                    "name": {
                        "type": "string"
                    }
                }
            }
        ))
    }
}