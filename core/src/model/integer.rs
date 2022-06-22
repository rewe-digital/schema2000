use crate::model::value_collection::ValueCollection;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct IntegerNode {
    pub values: ValueCollection<i64>,
}

impl IntegerNode {
    #[must_use]
    pub fn new() -> Self {
        Self {
            values: ValueCollection::new(),
        }
    }

    pub fn with_value(value: i64) -> Self {
        Self {
            values: ValueCollection::with_value(&value),
        }
    }

    pub fn with(values: ValueCollection<i64>) -> Self {
        Self { values }
    }
}

impl Default for IntegerNode {
    fn default() -> Self {
        IntegerNode::new()
    }
}
