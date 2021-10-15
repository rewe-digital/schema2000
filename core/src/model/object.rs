use std::collections::BTreeMap;

use crate::model::node_type::NodeType;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObjectProperty {
    pub node_type: NodeType,
    pub required: bool,
}

impl ObjectProperty {
    pub fn new<N: Into<NodeType>>(node_type: N) -> Self {
        ObjectProperty {
            node_type: node_type.into(),
            required: true,
        }
    }

    #[must_use]
    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }

    #[must_use]
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObjectNode {
    pub properties: BTreeMap<String, ObjectProperty>,
}

impl ObjectNode {
    #[must_use]
    pub fn new(properties: BTreeMap<String, ObjectProperty>) -> Self {
        ObjectNode { properties }
    }
}
