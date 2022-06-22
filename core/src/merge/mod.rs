use crate::merge::array::merge_array;
use crate::merge::object::merge_object;
use crate::model::NodeType::{Object, String};
use crate::model::{AnyNode, NodeType, ObjectNode, ObjectProperty, SchemaHypothesis, StringNode};
use crate::utils::SetVariances;
use integer::merge_integer;
use maplit::btreeset;
use std::collections::{BTreeSet, HashMap};
use std::iter::FromIterator;
use std::mem::Discriminant;
use string::merge_string;

mod any;
mod array;
mod integer;
mod object;
mod object_property;
mod string;

#[must_use]
pub fn merge_hypothesis(a: SchemaHypothesis, b: SchemaHypothesis) -> SchemaHypothesis {
    let root = merge_node_type(a.root, b.root);
    SchemaHypothesis { root }
}

#[must_use]
pub fn merge_hypothesis_disc(
    a: SchemaHypothesis,
    b: SchemaHypothesis,
    discrimiator: &str,
) -> SchemaHypothesis {
    // 1. Fall
    // a ist object
    // und hat property "discrimiator" mit Typ String und genau einen Wert in ValueCollection
    // und b ist object
    // und hat property "discrimiator" mit Typ String und genau einen Wert in ValueCollection
    // dann:
    // Wenn Werte identisch: normales merge
    // Wenn nicht: AnyOf aufbauen

    // 2. Fall
    // a ist anyOf
    // und es gibt objects mit property "discrimiator" mit Typ String und genau einen Wert in ValueCollection
    // und b ist object
    // und hat property "discrimiator" mit Typ String und genau einen Wert in ValueCollection
    // dann:
    // Wenn Wert von b in bestehend auftaucht: "damit" normales merge
    // Wenn nicht: AnyOf erweitern

    // in allen anderen Fällen: skip Merge

    let root = merge_node_type(a.root, b.root);
    SchemaHypothesis { root }
}

fn extract_discriminator_value(
    hypothesis: &SchemaHypothesis,
    discriminator: &str,
) -> Option<String> {
    match &hypothesis.root {
        Object(ObjectNode { properties }) => {
            let discriminator_property = properties.get(discriminator);
            match discriminator_property {
                Some(ObjectProperty { node_type, .. }) => match node_type {
                    String(StringNode { values }) => None,
                    _ => None,
                },
                _ => None,
            }
        }
        _ => None,
    }
}

pub fn merge_node_type(a: NodeType, b: NodeType) -> NodeType {
    match (a, b) {
        (a, b) if a == b => a,
        (NodeType::Integer(a), NodeType::Integer(b)) => merge_integer(a, b).into(),
        (NodeType::String(a), NodeType::String(b)) => merge_string(a, b).into(),
        (NodeType::Object(a), NodeType::Object(b)) => merge_object(a, b).into(),
        (NodeType::Array(a), NodeType::Array(b)) => merge_array(a, b).into(),
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

pub fn merge_node_types(node_types: &[NodeType]) -> NodeType {
    let mut merged_node_types: HashMap<Discriminant<NodeType>, NodeType> = HashMap::new();

    for node_type in node_types.iter().cloned() {
        let merged_node_type = match merged_node_types.get(&std::mem::discriminant(&node_type)) {
            None => node_type,
            Some(existing) => merge_node_type(existing.clone(), node_type),
        };
        merged_node_types.insert(std::mem::discriminant(&merged_node_type), merged_node_type);
    }

    let types = BTreeSet::from_iter(merged_node_types.values().into_iter().cloned());

    match SetVariances::new(&types) {
        SetVariances::Empty => unreachable!(),
        SetVariances::OneElement(node_type) => node_type.clone(),
        SetVariances::Multiple(_) => AnyNode::new(types).into(),
    }
}

#[cfg(test)]
mod test {
    use maplit::{btreemap, btreeset};

    use crate::merge::{extract_discriminator_value, merge_hypothesis, merge_node_type};
    use crate::model::{
        AnyNode, ArrayNode, IntegerNode, NodeType, ObjectNode, ObjectProperty, SchemaHypothesis,
        StringNode, ValueCollection,
    };

    #[test]
    fn test_merge_string() {
        let a = SchemaHypothesis::new(StringNode::with_value("a"));
        let b = SchemaHypothesis::new(StringNode::with_value("b"));

        let actual = merge_hypothesis(a, b);

        assert_eq!(
            actual,
            SchemaHypothesis::new(StringNode::with_values(vec!("a", "b")))
        );
    }

    #[test]
    fn test_merge_integer() {
        let a = SchemaHypothesis::new(IntegerNode::with_value(1));
        let b = SchemaHypothesis::new(IntegerNode::with_value(2));

        let actual = merge_hypothesis(a, b);

        assert_eq!(
            actual,
            SchemaHypothesis::new(IntegerNode::with(ValueCollection::with_values(vec!(1, 2))))
        );
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
        let a1 = StringNode::with_value("a1").into();
        let a2 = StringNode::with_value("a2").into();
        let b = IntegerNode::new().into();

        let actual = merge_node_type(a1, b);
        let actual = merge_node_type(actual, a2);

        assert_eq!(
            actual,
            AnyNode::new(btreeset![
                StringNode::with_values(vec!("a1", "a2")).into(),
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

    #[test]
    fn test_extract_discriminator_value() {
        let a = SchemaHypothesis::new(ObjectNode::new(btreemap! {
            String::from("type") => ObjectProperty::new(StringNode::with_value("a")),
            String::from("name") => ObjectProperty::new(StringNode::new())
        }));
        let actual = extract_discriminator_value(&a, "type");
        assert_eq!(Some("a".to_string()), actual)
    }
}
