use std::collections::BTreeSet;
use std::iter::FromIterator;
use crate::model::IntegerNode;

pub fn merge_integer(i1: IntegerNode, i2: IntegerNode) -> IntegerNode {
    IntegerNode {
        values: BTreeSet::from_iter(i1.values.union(&i2.values).cloned()),
    }
}
