use crate::model::value_collection::ValueCollection;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringNode {
    pub values: ValueCollection<String>,
}

impl StringNode {
    #[must_use]
    pub fn new() -> Self {
        Self {
            values: ValueCollection::new(),
        }
    }

    #[must_use]
    pub fn with_value(value: &str) -> Self {
        Self {
            // TODO why is this not assignment-compatible??
            values: ValueCollection::with_value(&value.to_string()),
        }
    }

    #[must_use]
    pub fn with_values(values: Vec<&str>) -> Self {
        Self {
            values: ValueCollection::with_values(values.into_iter().map(|s| s.into()).collect()),
        }
    }
}

impl Default for StringNode {
    fn default() -> Self {
        StringNode::new()
    }
}
