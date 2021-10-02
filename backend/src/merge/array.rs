use crate::merge::merge_node_type;
use crate::model::ArrayNode;

pub fn merge_array(a: ArrayNode, b: ArrayNode) -> ArrayNode {
    match (a, b) {
        (ArrayNode { items: None }, ys) => ys,
        (xs, ArrayNode { items: None }) => xs,
        (ArrayNode { items: Some(xs) }, ArrayNode { items: Some(ys) }) => {
            ArrayNode::new(merge_node_type(*xs, *ys))
        }
    }
}
