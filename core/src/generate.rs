use std::collections::{BTreeMap, BTreeSet};

use crate::merge_hypothesis;
use serde_json::{Map, Value};

use crate::model::{
    AnyNode, ArrayNode, IntegerNode, NodeType, NumberNode, ObjectNode, ObjectProperty,
    SchemaHypothesis, StringNode,
};
use crate::utils::SetVariances;

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
        Value::Number(_) => IntegerNode::new().into(),
        Value::String(_) => StringNode::new().into(),
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
    let mut merged_obj_type: Option<NodeType> = None;
    let mut merged_array_type: Option<NodeType> = None;
    let mut types = BTreeSet::new();

    for value in array_values.iter() {
        let value_type = generate_node_type(value);
        match value_type {
            NodeType::Object(ObjectNode { properties: _ }) => {
                merged_obj_type = match merged_obj_type {
                    Some(acc) => Some(crate::merge::merge_node_type(acc, value_type)),
                    None => Some(value_type),
                };
            }
            NodeType::Array(_) => {
                merged_array_type = match merged_array_type {
                    Some(acc) => Some(crate::merge::merge_node_type(acc, value_type)),
                    None => Some(value_type),
                }
            }
            _ => {
                types.insert(value_type);
            }
        };
    }
    if let Some(node_type) = merged_obj_type {
        types.insert(node_type);
    }

    if let Some(node_type) = merged_array_type {
        types.insert(node_type);
    }

    match SetVariances::new(&types) {
        SetVariances::Empty => unreachable!(),
        SetVariances::OneElement(node_type) => node_type.clone(),
        SetVariances::Multiple(_) => AnyNode::new(types).into(),
    }
}

#[must_use]
pub fn generate_hypothesis(dom: &Value) -> SchemaHypothesis {
    SchemaHypothesis {
        root: generate_node_type(dom),
    }
}

pub fn generate_hypothesis_from_iterator<I>(values: I) -> SchemaHypothesis
where
    I: Iterator<Item = Value>,
{
    let mut current_hypothesis: Option<SchemaHypothesis> = None;

    for json_document in values {
        let new_hypo = generate_hypothesis(&json_document);
        if current_hypothesis.is_none() {
            current_hypothesis = Some(new_hypo);
        } else {
            current_hypothesis = current_hypothesis.map(|cur| merge_hypothesis(cur, new_hypo));
        }
    }

    current_hypothesis.unwrap()
}

#[cfg(test)]
mod test {
    use maplit::{btreemap, btreeset};
    use serde_json::json;

    use crate::generate::generate_node_type;
    use crate::model::{
        AnyNode, ArrayNode, IntegerNode, NodeType, NumberNode, ObjectNode, ObjectProperty,
        StringNode,
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
        assert_eq!(generate_node_type(&dom), IntegerNode::new().into());
    }

    #[test]
    fn test_number() {
        let dom = json!(10.5);
        assert_eq!(generate_node_type(&dom), NumberNode::new().into());
    }

    #[test]
    fn test_string() {
        let dom = json!("Schema 2000");
        assert_eq!(generate_node_type(&dom), StringNode::new().into());
    }

    #[test]
    fn test_array_merge_objects() {
        let dom = json!(["one", 1, {"a": 1}, {"a": "1"}]);
        let actual = generate_node_type(&dom);
        let expected = ArrayNode::new_many(btreeset! {
            StringNode::new().into(),
            IntegerNode::new().into(),
            ObjectNode::new(btreemap! {
                    "a".to_string() => ObjectProperty { required: true, node_type: AnyNode::new(
                        btreeset! { StringNode::new().into(), IntegerNode::new().into() }
                    ).into()}
                }).into()
        })
        .into();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_array_all_int() {
        let dom = json!([10, 15, 25]);
        assert_eq!(
            generate_node_type(&dom),
            ArrayNode::new(IntegerNode::new().into()).into()
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
                IntegerNode::new().into(),
                StringNode::new().into()
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
            "name".to_string() => ObjectProperty::new(StringNode::new()),
            "length".to_string() => ObjectProperty::new(IntegerNode::new()),
        })
        .into();

        assert_eq!(generate_node_type(&dom), expected);
    }
}
