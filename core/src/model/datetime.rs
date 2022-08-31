#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTimeNode {}

impl DateTimeNode {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for DateTimeNode {
    fn default() -> Self {
        DateTimeNode::new()
    }
}
