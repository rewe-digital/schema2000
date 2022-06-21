use maplit::btreeset;
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct IntegerNode {
    pub values: BTreeSet<i64>,
}

impl IntegerNode {
    #[must_use]
    pub fn new() -> Self {
        Self {
            values: BTreeSet::new(),
        }
    }

    pub fn with_value(value: i64) -> Self {
        Self {
            values: btreeset![value],
        }
    }
}

impl Default for IntegerNode {
    fn default() -> Self {
        IntegerNode::new()
    }
}
