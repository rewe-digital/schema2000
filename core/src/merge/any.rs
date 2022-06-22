use crate::merge::merge_node_types;
use crate::model::{AnyNode, NodeType};

pub fn merge_any(xs: &AnyNode, ys: AnyNode) -> NodeType {
    let unified_types: Vec<NodeType> = xs.nodes.union(&ys.nodes).cloned().collect();
    merge_node_types(&unified_types)
}
