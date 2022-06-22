use crate::model::IntegerNode;

pub fn merge_integer(i1: IntegerNode, i2: IntegerNode) -> IntegerNode {
    IntegerNode {
        values: i1.values.merge(&i2.values),
    }
}
