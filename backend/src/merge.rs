use std::collections::{BTreeSet, HashSet};

use maplit::btreeset;

use crate::SchemaHypothesis;
use crate::{NodeType, ObjectProperty};

#[must_use]
pub fn merge_hypothesis(a: SchemaHypothesis, b: SchemaHypothesis) -> SchemaHypothesis {
    let root = merge_node_type(a.root, b.root);
    SchemaHypothesis { root }
}

pub fn merge_node_type(a: NodeType, b: NodeType) -> NodeType {
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
                        (*key).to_string(),
                        merge_object_property(properties_a.get(*key), properties_b.get(*key)),
                    )
                })
                .collect();
            NodeType::Object {
                properties: merged_properties,
            }
        }
        (NodeType::Array(None), ys @ NodeType::Array(_)) => ys,
        (xs @ NodeType::Array(_), NodeType::Array(None)) => xs,
        (NodeType::Array(Some(xs)), NodeType::Array(Some(ys))) => {
            NodeType::Array(Some(Box::new(merge_node_type(*xs, *ys))))
        }
        (NodeType::Any(xs), NodeType::Any(ys)) => merge_any(&xs, ys),
        (a @ NodeType::Any(_), b) | (b, a @ NodeType::Any(_)) => {
            merge_node_type(a, NodeType::Any(btreeset![b]))
        }
        (a, b) => merge_node_type(NodeType::Any(btreeset![a]), NodeType::Any(btreeset![b])),
    }
}

fn merge_any(xs: &BTreeSet<NodeType>, ys: BTreeSet<NodeType>) -> NodeType {
    let mut zs = xs.clone();
    for node_type in ys {
        match node_type {
            node @ NodeType::Object { .. } => match xs.iter().find(|x| x.is_object()) {
                None => {
                    zs.insert(node);
                }
                Some(other @ NodeType::Object { .. }) => {
                    zs.remove(other);
                    zs.insert(merge_node_type(node, other.clone()));
                }
                Some(_) => unreachable!(),
            },
            node @ NodeType::Array(_) => match xs.iter().find(|x| x.is_array()) {
                None => {
                    zs.insert(node);
                }
                Some(other @ NodeType::Array(_)) => {
                    zs.remove(other);
                    zs.insert(merge_node_type(node, other.clone()));
                }
                Some(_) => unreachable!(),
            },
            _ => {
                zs.insert(node_type);
            }
        }
    }
    NodeType::Any(zs)
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
    use maplit::{btreemap, btreeset};

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
        );
    }

    #[test]
    fn test_merge_array_without_types() {
        let a = NodeType::new_untyped_array();
        let b = NodeType::new_untyped_array();

        assert_eq!(merge_node_type(a, b), NodeType::new_untyped_array());
    }

    #[test]
    fn test_merge_array_with_same_types() {
        let a = NodeType::new_typed_array(btreeset!(NodeType::Integer));
        let b = NodeType::new_typed_array(btreeset!(NodeType::Integer));

        assert_eq!(
            merge_node_type(a, b),
            NodeType::new_typed_array(btreeset!(NodeType::Integer))
        );
    }

    #[test]
    fn test_merge_array_with_one_empty_one_given() {
        let a = NodeType::new_untyped_array();
        let b = NodeType::new_typed_array(btreeset!(NodeType::Integer));

        assert_eq!(
            merge_node_type(a, b),
            NodeType::new_typed_array(btreeset!(NodeType::Integer))
        );
    }

    #[test]
    fn test_merge_array_with_different_types() {
        let a = NodeType::new_typed_array(btreeset![NodeType::Integer, NodeType::String]);
        let b = NodeType::new_typed_array(btreeset![NodeType::Integer, NodeType::Boolean]);

        assert_eq!(
            merge_node_type(a, b),
            NodeType::new_typed_array(btreeset![
                NodeType::Integer,
                NodeType::String,
                NodeType::Boolean
            ])
        );
    }

    #[test]
    fn test_merge_array_with_objects() {
        let a = NodeType::new_typed_array(btreeset![NodeType::Object {
            properties: btreemap! {
                "id".to_string() => ObjectProperty {
                    node_type: NodeType::Integer,
                    required: true
                }
            }
        }]);
        let b = NodeType::new_typed_array(btreeset![NodeType::Object {
            properties: btreemap! {
                "name".to_string() => ObjectProperty {
                    node_type: NodeType::String,
                    required: true
                }
            }
        }]);

        assert_eq!(
            merge_node_type(a, b),
            NodeType::new_typed_array(btreeset![NodeType::Object {
                properties: btreemap! {
                    "id".to_string() => ObjectProperty {
                        node_type: NodeType::Integer,
                        required: false
                    },
                    "name".to_string() => ObjectProperty {
                        node_type: NodeType::String,
                        required: false
                    }
                }
            }])
        );
    }

    #[test]
    fn test_merge_object_additional_property_b() {
        let a = SchemaHypothesis {
            root: NodeType::Object {
                properties: btreemap! {
                    String::from("id") => ObjectProperty { required: true, node_type: NodeType::String }
                },
            },
        };

        let b = SchemaHypothesis {
            root: NodeType::Object {
                properties: btreemap! {
                    String::from("id") => ObjectProperty { required: true, node_type: NodeType::String },
                    String::from("name") => ObjectProperty { required: true, node_type: NodeType::String }
                },
            },
        };

        let actual = merge_hypothesis(a, b);
        let expected = SchemaHypothesis {
            root: NodeType::Object {
                properties: btreemap! {
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
                properties: btreemap! {
                    String::from("id") => ObjectProperty { required: true, node_type: NodeType::String },
                    String::from("name") => ObjectProperty { required: true, node_type: NodeType::String }
                },
            },
        };

        let b = SchemaHypothesis {
            root: NodeType::Object {
                properties: btreemap! {
                    String::from("id") => ObjectProperty { required: true, node_type: NodeType::String },
                },
            },
        };

        let actual = merge_hypothesis(a, b);
        let expected = SchemaHypothesis {
            root: NodeType::Object {
                properties: btreemap! {
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
            NodeType::Any(btreeset![NodeType::String, NodeType::Integer])
        );
    }

    #[test]
    fn test_merge_any_and_type() {
        let a = NodeType::Any(btreeset![NodeType::Integer]);
        let b = NodeType::String;

        let actual = merge_node_type(a, b);

        assert_eq!(
            actual,
            NodeType::Any(btreeset![NodeType::Integer, NodeType::String])
        );
    }

    #[test]
    fn test_merge_type_and_any() {
        let a = NodeType::String;
        let b = NodeType::Any(btreeset![NodeType::Integer]);

        let actual = merge_node_type(a, b);

        assert_eq!(
            actual,
            NodeType::Any(btreeset![NodeType::Integer, NodeType::String])
        );
    }

    #[test]
    fn test_merge_existing_type_and_any() {
        let a = NodeType::Any(btreeset![NodeType::String, NodeType::Integer]);
        let b = NodeType::String;

        let actual = merge_node_type(a, b);

        assert_eq!(
            actual,
            NodeType::Any(btreeset![NodeType::String, NodeType::Integer])
        );
    }
}
