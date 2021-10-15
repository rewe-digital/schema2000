#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct IntegerNode {}

impl IntegerNode {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for IntegerNode {
    fn default() -> Self {
        IntegerNode::new()
    }
}
