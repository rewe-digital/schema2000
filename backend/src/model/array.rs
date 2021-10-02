use crate::model::any::AnyNode;
use crate::model::node_type::NodeType;
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArrayNode {
    pub items: Option<Box<NodeType>>,
}

impl ArrayNode {
    #[must_use]
    pub fn new(node_type: NodeType) -> Self {
        Self {
            items: Some(Box::new(node_type)),
        }
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn new_many(node_types: BTreeSet<NodeType>) -> Self {
        match node_types.len() {
            0 => Self { items: None },
            1 => Self {
                items: Some(Box::new(node_types.into_iter().next().unwrap())),
            },
            _ => Self {
                items: Some(Box::new(NodeType::Any(AnyNode::new(node_types)))),
            },
        }
    }

    #[must_use]
    pub fn new_untyped() -> Self {
        Self { items: None }
    }
}
