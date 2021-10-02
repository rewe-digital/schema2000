#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumberNode {}

impl NumberNode {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for NumberNode {
    fn default() -> Self {
        NumberNode::new()
    }
}
