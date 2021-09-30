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
    #[must_use]
    pub fn is_object(&self) -> bool {
        matches!(self, NodeType::Object { .. })
    }

    #[must_use]
    pub fn is_array(&self) -> bool {
        matches!(self, NodeType::Array(_))
    }

    #[must_use]
    pub fn new_untyped_array() -> Self {
        Self::new_typed_array(BTreeSet::new())
    }

    #[must_use]
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
