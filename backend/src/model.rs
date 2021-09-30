use std::collections::{BTreeMap, BTreeSet};

use crate::utils::SetVariances;

#[derive(Debug, PartialEq)]
pub struct SchemaHypothesis {
    pub root: NodeType,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObjectProperty {
    pub node_type: NodeType,
    pub required: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NodeType {
    String,
    Integer,
    Number,
    Boolean,
    Null,
    Array(Option<Box<NodeType>>),
    Object {
        properties: BTreeMap<String, ObjectProperty>,
    },
    Any(BTreeSet<NodeType>),
}

impl NodeType {
    pub fn new_untyped_array() -> Self {
        Self::new_typed_array(BTreeSet::new())
    }
    pub fn new_typed_array(node_types: BTreeSet<NodeType>) -> Self {
        match SetVariances::new(&node_types) {
            SetVariances::Empty => NodeType::Array(None),
            SetVariances::OneElement(node_type) => {
                NodeType::Array(Some(Box::new(node_type.clone())))
            }
            SetVariances::Multiple(_) => NodeType::Array(Some(Box::new(NodeType::Any(node_types)))),
        }
    }
}
