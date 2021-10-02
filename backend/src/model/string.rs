#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringNode {}

impl StringNode {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for StringNode {
    fn default() -> Self {
        StringNode::new()
    }
}
