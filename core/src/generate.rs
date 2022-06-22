use std::collections::BTreeMap;

use serde_json::{Map, Value};

use crate::merge;
use crate::model::{
    ArrayNode, IntegerNode, NodeType, NumberNode, ObjectNode, ObjectProperty, SchemaHypothesis,
    StringNode,
};

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
        Value::Number(i) if i.is_f64() => NumberNode::new().into(),
        Value::Number(i) => IntegerNode::with_value(i.as_i64().unwrap()).into(),
        Value::String(s) => StringNode::with_value(s).into(),
        Value::Array(array_values) => {
            if array_values.is_empty() {
                ArrayNode::new_untyped().into()
            } else {
                ArrayNode::new(generate_node_type_for_array_values(array_values)).into()
            }
        }
        Value::Object(props) => ObjectNode::new(generate_properties(props)).into(),
    }
}

fn generate_node_type_for_array_values(array_values: &[Value]) -> NodeType {
    let node_types: Vec<NodeType> = array_values.iter().map(generate_node_type).collect();
    merge::merge_node_types(&node_types)
}

#[must_use]
pub fn generate_hypothesis(dom: &Value) -> SchemaHypothesis {
    SchemaHypothesis {
        root: generate_node_type(dom),
    }
}

#[cfg(test)]
mod test {
    use maplit::{btreemap, btreeset};
    use serde_json::json;

    use crate::generate::generate_node_type;
    use crate::model::{
        AnyNode, ArrayNode, IntegerNode, NodeType, NumberNode, ObjectNode, ObjectProperty,
        StringNode, ValueCollection,
    };

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
        assert_eq!(generate_node_type(&dom), IntegerNode::with_value(10).into());
    }

    #[test]
    fn test_number() {
        let dom = json!(10.5);
        assert_eq!(generate_node_type(&dom), NumberNode::new().into());
    }

    #[test]
    fn test_string() {
        let dom = json!("Schema 2000");
        assert_eq!(
            generate_node_type(&dom),
            StringNode::with_value("Schema 2000").into()
        );
    }

    #[test]
    fn test_array_merge_objects() {
        let dom = json!(["one", "two", 1, 2, {"a": 1}, {"a": "1"}]);
        let actual = generate_node_type(&dom);
        let expected = ArrayNode::new_many(btreeset! {
            StringNode::with_values(vec!("one", "two")).into(),
            IntegerNode::with(ValueCollection::with_values(vec!(1,2))).into(),
            ObjectNode::new(btreemap! {
                    "a".to_string() => ObjectProperty { required: true, node_type: AnyNode::new(
                        btreeset! { StringNode::with_value("1").into(), IntegerNode::with_value(1).into() }
                    ).into()}
                }).into()
        }).into();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_array_all_int() {
        let dom = json!([10, 15, 25]);
        assert_eq!(
            generate_node_type(&dom),
            ArrayNode::new(
                IntegerNode::with(ValueCollection::with_values(vec!(10, 15, 25))).into()
            )
            .into()
        );
    }

    #[test]
    fn test_array_empty() {
        let dom = json!([]);
        assert_eq!(generate_node_type(&dom), ArrayNode::new_untyped().into());
    }

    #[test]
    fn test_array_int_and_string() {
        let dom = json!([42, "Hello"]);

        assert_eq!(
            generate_node_type(&dom),
            ArrayNode::new_many(btreeset![
                IntegerNode::with_value(42).into(),
                StringNode::with_value("Hello").into()
            ])
            .into()
        );
    }

    #[test]
    fn test_object() {
        let dom = json!({
            "name": "Schokoladenbrunnen",
            "length": 100
        });
        let expected = ObjectNode::new(btreemap! {
            "name".to_string() => ObjectProperty::new(StringNode::with_value("Schokoladenbrunnen")),
            "length".to_string() => ObjectProperty::new(IntegerNode::with_value(100)),
        })
        .into();

        assert_eq!(generate_node_type(&dom), expected);
    }
}
