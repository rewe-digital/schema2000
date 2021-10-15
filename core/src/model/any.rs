use std::collections::BTreeSet;

use crate::model::node_type::NodeType;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AnyNode {
    pub nodes: BTreeSet<NodeType>,
}

impl AnyNode {
    #[must_use]
    pub fn new(nodes: BTreeSet<NodeType>) -> Self {
        Self { nodes }
    }
}

impl Default for AnyNode {
    fn default() -> Self {
        AnyNode::new(BTreeSet::new())
    }
}
