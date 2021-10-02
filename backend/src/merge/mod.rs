use crate::model::{AnyNode, ArrayNode, NodeType, ObjectNode, SchemaHypothesis};
use maplit::btreeset;
use std::collections::HashSet;

mod any;
mod object_property;

#[must_use]
pub fn merge_hypothesis(a: SchemaHypothesis, b: SchemaHypothesis) -> SchemaHypothesis {
    let root = merge_node_type(a.root, b.root);
    SchemaHypothesis { root }
}

pub fn merge_node_type(a: NodeType, b: NodeType) -> NodeType {
    match (a, b) {
        (a, b) if a == b => a,
        (
            NodeType::Object(ObjectNode {
                properties: properties_a,
            }),
            NodeType::Object(ObjectNode {
                properties: properties_b,
            }),
        ) => {
            let keys_a: HashSet<&String> = properties_a.keys().collect();
            let keys_b: HashSet<&String> = properties_b.keys().collect();
            let merged_properties = keys_a
                .union(&keys_b)
                .map(|key| {
                    (
                        (*key).to_string(),
                        object_property::merge_object_property(
                            properties_a.get(*key),
                            properties_b.get(*key),
                        ),
                    )
                })
                .collect();
            NodeType::Object(ObjectNode {
                properties: merged_properties,
            })
        }
        (NodeType::Array(ArrayNode { items: None }), ys @ NodeType::Array(_)) => ys,
        (xs @ NodeType::Array(_), NodeType::Array(ArrayNode { items: None })) => xs,
        (
            NodeType::Array(ArrayNode { items: Some(xs) }),
            NodeType::Array(ArrayNode { items: Some(ys) }),
        ) => ArrayNode::new(merge_node_type(*xs, *ys)).into(),
        (NodeType::Any(xs), NodeType::Any(ys)) => any::merge_any(&xs, ys),
        (a @ NodeType::Any(_), b) | (b, a @ NodeType::Any(_)) => {
            merge_node_type(a, AnyNode::new(btreeset![b]).into())
        }
        (a, b) => merge_node_type(
            AnyNode::new(btreeset![a]).into(),
            AnyNode::new(btreeset![b]).into(),
        ),
    }
}

#[cfg(test)]
mod test {
    use maplit::{btreemap, btreeset};

    use crate::merge::{merge_hypothesis, merge_node_type};
    use crate::model::{
        AnyNode, ArrayNode, IntegerNode, NodeType, ObjectNode, ObjectProperty, SchemaHypothesis,
        StringNode,
    };

    #[test]
    fn test_merge_string() {
        let a = SchemaHypothesis::new(StringNode::new());
        let b = SchemaHypothesis::new(StringNode::new());

        let actual = merge_hypothesis(a, b);

        assert_eq!(actual, SchemaHypothesis::new(StringNode::new()));
    }

    #[test]
    fn test_merge_array_without_types() {
        let a = ArrayNode::new_untyped();
        let b = ArrayNode::new_untyped();

        assert_eq!(
            merge_node_type(a.into(), b.into()),
            ArrayNode::new_untyped().into()
        );
    }

    #[test]
    fn test_merge_array_with_same_types() {
        let a = ArrayNode::new_many(btreeset!(IntegerNode::new().into()));
        let b = ArrayNode::new_many(btreeset!(IntegerNode::new().into()));

        assert_eq!(
            merge_node_type(a.into(), b.into()),
            ArrayNode::new_many(btreeset!(IntegerNode::new().into())).into()
        );
    }

    #[test]
    fn test_merge_array_with_one_empty_one_given() {
        let a = ArrayNode::new_untyped();
        let b = ArrayNode::new_many(btreeset!(IntegerNode::new().into()));

        assert_eq!(
            merge_node_type(a.into(), b.into()),
            ArrayNode::new_many(btreeset!(IntegerNode::new().into())).into()
        );
    }

    #[test]
    fn test_merge_array_with_different_types() {
        let a = ArrayNode::new_many(btreeset![
            IntegerNode::new().into(),
            StringNode::new().into()
        ])
        .into();
        let b = ArrayNode::new_many(btreeset![IntegerNode::new().into(), NodeType::Boolean]).into();

        assert_eq!(
            merge_node_type(a, b),
            ArrayNode::new_many(btreeset![
                IntegerNode::new().into(),
                StringNode::new().into(),
                NodeType::Boolean
            ])
            .into()
        );
    }

    #[test]
    fn test_merge_array_with_objects() {
        let a = ArrayNode::new_many(btreeset![ObjectNode::new(btreemap! {
            "id".to_string() => ObjectProperty {
                node_type: IntegerNode::new().into(),
                required: true
            }
        })
        .into()]);
        let b = ArrayNode::new_many(btreeset![ObjectNode::new(btreemap! {
            "name".to_string() => ObjectProperty {
                node_type: StringNode::new().into(),
                required: true
            }
        })
        .into()]);

        assert_eq!(
            merge_node_type(a.into(), b.into()),
            ArrayNode::new_many(btreeset![ObjectNode::new(btreemap! {
                "id".to_string() => ObjectProperty {
                    node_type: IntegerNode::new().into(),
                    required: false
                },
                "name".to_string() => ObjectProperty {
                    node_type: StringNode::new().into(),
                    required: false
                }
            })
            .into()])
            .into()
        );
    }

    #[test]
    fn test_merge_object_additional_property_b() {
        let a = SchemaHypothesis::new(ObjectNode::new(btreemap! {
            String::from("id") => ObjectProperty::new(StringNode::new())
        }));

        let b = SchemaHypothesis::new(ObjectNode::new(btreemap! {
            String::from("id") => ObjectProperty::new(StringNode::new()),
            String::from("name") => ObjectProperty::new(StringNode::new())
        }));

        let actual = merge_hypothesis(a, b);

        let expected = SchemaHypothesis::new(ObjectNode::new(btreemap! {
            String::from("id") => ObjectProperty::new(StringNode::new()),
            String::from("name") => ObjectProperty::new(StringNode::new()).optional()
        }));

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_merge_object_property_missing_in_b() {
        let a = SchemaHypothesis::new(ObjectNode::new(btreemap! {
            String::from("id") => ObjectProperty::new(StringNode::new()),
            String::from("name") => ObjectProperty::new(StringNode::new())
        }));

        let b = SchemaHypothesis::new(ObjectNode::new(btreemap! {
            String::from("id") => ObjectProperty::new(StringNode::new()),
        }));

        let actual = merge_hypothesis(a, b);
        let expected = SchemaHypothesis::new(ObjectNode::new(btreemap! {
            String::from("id") => ObjectProperty::new(StringNode::new()),
            String::from("name") => ObjectProperty::new(StringNode::new()).optional()
        }));

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_merge_different_types() {
        let a = StringNode::new().into();
        let b = IntegerNode::new().into();

        let actual = merge_node_type(a, b);

        assert_eq!(
            actual,
            AnyNode::new(btreeset![
                StringNode::new().into(),
                IntegerNode::new().into()
            ])
            .into()
        );
    }

    #[test]
    fn test_merge_any_and_type() {
        let a = AnyNode::new(btreeset![IntegerNode::new().into()]).into();
        let b = StringNode::new().into();

        let actual = merge_node_type(a, b);

        assert_eq!(
            actual,
            AnyNode::new(btreeset![
                IntegerNode::new().into(),
                StringNode::new().into()
            ])
            .into()
        );
    }

    #[test]
    fn test_merge_type_and_any() {
        let a = StringNode::new().into();
        let b = AnyNode::new(btreeset![IntegerNode::new().into()]).into();

        let actual = merge_node_type(a, b);

        assert_eq!(
            actual,
            AnyNode::new(btreeset![
                IntegerNode::new().into(),
                StringNode::new().into()
            ])
            .into()
        );
    }

    #[test]
    fn test_merge_existing_type_and_any() {
        let a = AnyNode::new(btreeset![
            StringNode::new().into(),
            IntegerNode::new().into()
        ])
        .into();
        let b = StringNode::new().into();

        let actual = merge_node_type(a, b);

        assert_eq!(
            actual,
            AnyNode::new(btreeset![
                StringNode::new().into(),
                IntegerNode::new().into()
            ])
            .into()
        );
    }
}
