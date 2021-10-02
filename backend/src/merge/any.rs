use crate::merge;
use crate::model::{AnyNode, NodeType};

pub fn merge_any(xs: &AnyNode, ys: AnyNode) -> NodeType {
    let xs = &xs.nodes;
    let ys = ys.nodes;

    let mut zs = xs.clone();
    for node_type in ys {
        match node_type {
            node @ NodeType::Object { .. } => match xs.iter().find(|x| x.is_object()) {
                None => {
                    zs.insert(node);
                }
                Some(other @ NodeType::Object { .. }) => {
                    zs.remove(other);
                    zs.insert(merge::merge_node_type(node, other.clone()));
                }
                Some(_) => unreachable!(),
            },
            node @ NodeType::Array(_) => match xs.iter().find(|x| x.is_array()) {
                None => {
                    zs.insert(node);
                }
                Some(other @ NodeType::Array(_)) => {
                    zs.remove(other);
                    zs.insert(merge::merge_node_type(node, other.clone()));
                }
                Some(_) => unreachable!(),
            },
            _ => {
                zs.insert(node_type);
            }
        }
    }

    AnyNode::new(zs).into()
}
