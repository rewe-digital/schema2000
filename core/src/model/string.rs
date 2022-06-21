use maplit::btreeset;
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringNode {
    pub values: BTreeSet<String>,
}

impl StringNode {
    #[must_use]
    pub fn new() -> Self {
        Self {
            values: BTreeSet::new(),
        }
    }

    #[must_use]
    pub fn with_value(value: &str) -> Self {
        Self {
            values: btreeset![value.to_string()],
        }
    }
}

impl Default for StringNode {
    fn default() -> Self {
        StringNode::new()
    }
}
