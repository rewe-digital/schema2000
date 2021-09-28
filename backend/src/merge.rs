use std::collections::HashSet;

use crate::NodeType::Any;
use crate::SchemaHypothesis;
use crate::{NodeType, ObjectProperty};

pub fn merge_hypothesis(a: SchemaHypothesis, b: SchemaHypothesis) -> SchemaHypothesis {
    let root = merge_node_type(a.root, b.root);
    SchemaHypothesis { root }
}

fn merge_node_type(a: NodeType, b: NodeType) -> NodeType {
    match (a, b) {
        (a, b) if a == b => a,
        (
            NodeType::Object {
                properties: properties_a,
            },
            NodeType::Object {
                properties: properties_b,
            },
        ) => {
            let keys_a: HashSet<&String> = properties_a.keys().collect();
            let keys_b: HashSet<&String> = properties_b.keys().collect();
            let merged_properties = keys_a
                .union(&keys_b)
                .map(|key| {
                    (
                        key.to_string(),
                        merge_object_property(properties_a.get(*key), properties_b.get(*key)),
                    )
                })
                .collect();
            NodeType::Object {
                properties: merged_properties,
            }
        }
        (NodeType::Any(xs), NodeType::Any(ys)) => {
            let merged_types = ys.into_iter().fold(xs, |mut acc, y| {
                if !acc.contains(&y) {
                    acc.push(y);
                }

                acc
            });
            Any(merged_types)
        }
        (a @ NodeType::Any(_), b) | (b, a @ NodeType::Any(_)) => {
            merge_node_type(a, NodeType::Any(vec![b]))
        }
        (a, b) => merge_node_type(NodeType::Any(vec![a]), NodeType::Any(vec![b])),
    }
}

fn merge_object_property(a: Option<&ObjectProperty>, b: Option<&ObjectProperty>) -> ObjectProperty {
    match (a, b) {
        (Some(a), None) => ObjectProperty {
            required: false,
            ..a.clone()
        },
        (None, Some(b)) => ObjectProperty {
            required: false,
            ..b.clone()
        },
        (Some(a), Some(b)) => ObjectProperty {
            required: a.required && b.required,
            node_type: merge_node_type(a.clone().node_type, b.clone().node_type),
        },
        (None, None) => unreachable!(),
    }
}

#[cfg(test)]
mod test {
    use maplit::hashmap;

    use crate::merge::{merge_hypothesis, merge_node_type};
    use crate::ObjectProperty;
    use crate::{NodeType, SchemaHypothesis};

    #[test]
    fn test_merge_string() {
        let a = SchemaHypothesis {
            root: NodeType::String,
        };
        let b = SchemaHypothesis {
            root: NodeType::String,
        };

        let actual = merge_hypothesis(a, b);

        assert_eq!(
            actual,
            SchemaHypothesis {
                root: NodeType::String
            }
        )
    }

    #[test]
    fn test_merge_object_additional_property_b() {
        let a = SchemaHypothesis {
            root: NodeType::Object {
                properties: hashmap! {
                    String::from("id") => ObjectProperty { required: true, node_type: NodeType::String }
                },
            },
        };

        let b = SchemaHypothesis {
            root: NodeType::Object {
                properties: hashmap! {
                    String::from("id") => ObjectProperty { required: true, node_type: NodeType::String },
                    String::from("name") => ObjectProperty { required: true, node_type: NodeType::String }
                },
            },
        };

        let actual = merge_hypothesis(a, b);
        let expected = SchemaHypothesis {
            root: NodeType::Object {
                properties: hashmap! {
                    String::from("id") => ObjectProperty { required: true, node_type: NodeType::String },
                    String::from("name") => ObjectProperty { required: false, node_type: NodeType::String }
                },
            },
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_merge_object_property_missing_in_b() {
        let a = SchemaHypothesis {
            root: NodeType::Object {
                properties: hashmap! {
                    String::from("id") => ObjectProperty { required: true, node_type: NodeType::String },
                    String::from("name") => ObjectProperty { required: true, node_type: NodeType::String }
                },
            },
        };

        let b = SchemaHypothesis {
            root: NodeType::Object {
                properties: hashmap! {
                    String::from("id") => ObjectProperty { required: true, node_type: NodeType::String },
                },
            },
        };

        let actual = merge_hypothesis(a, b);
        let expected = SchemaHypothesis {
            root: NodeType::Object {
                properties: hashmap! {
                    String::from("id") => ObjectProperty { required: true, node_type: NodeType::String },
                    String::from("name") => ObjectProperty { required: false, node_type: NodeType::String }
                },
            },
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_merge_different_types() {
        let a = NodeType::String;
        let b = NodeType::Integer;

        let actual = merge_node_type(a, b);

        assert_eq!(
            actual,
            NodeType::Any(vec![NodeType::String, NodeType::Integer])
        )
    }

    #[test]
    fn test_merge_any_and_type() {
        let a = NodeType::Any(vec![NodeType::Integer]);
        let b = NodeType::String;

        let actual = merge_node_type(a, b);

        assert_eq!(
            actual,
            NodeType::Any(vec![NodeType::Integer, NodeType::String])
        )
    }

    #[test]
    fn test_merge_type_and_any() {
        let a = NodeType::String;
        let b = NodeType::Any(vec![NodeType::Integer]);

        let actual = merge_node_type(a, b);

        assert_eq!(
            actual,
            NodeType::Any(vec![NodeType::Integer, NodeType::String])
        )
    }

    #[test]
    fn test_merge_existing_type_and_any() {
        let a = NodeType::Any(vec![NodeType::String, NodeType::Integer]);
        let b = NodeType::String;

        let actual = merge_node_type(a, b);

        assert_eq!(
            actual,
            NodeType::Any(vec![NodeType::String, NodeType::Integer])
        )
    }
}
