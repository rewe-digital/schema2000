use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, PartialEq)]
pub struct SchemaHypothesis {
    pub root: NodeType,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObjectProperty {
    pub node_type: NodeType,
    pub required: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NodeType {
    String,
    Integer,
    Number,
    Boolean,
    Null,
    Array(BTreeSet<NodeType>),
    Object {
        properties: BTreeMap<String, ObjectProperty>,
    },
    Any(BTreeSet<NodeType>),
}
