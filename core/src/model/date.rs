#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateNode {}

impl DateNode {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for DateNode {
    fn default() -> Self {
        DateNode::new()
    }
}
