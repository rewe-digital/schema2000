use std::collections::BTreeSet;
use std::iter::FromIterator;
use crate::model::StringNode;

pub fn merge_string(s1: StringNode, s2: StringNode) -> StringNode {
    StringNode {
        values: BTreeSet::from_iter(s1.values.union(&s2.values).cloned()),
    }
}
