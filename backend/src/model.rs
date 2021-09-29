use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct SchemaHypothesis {
    pub root: NodeType,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectProperty {
    pub node_type: NodeType,
    pub required: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NodeType {
    String,
    Integer,
    Number,
    Boolean,
    Null,
    // TODO Use a HashSet. Problem: Neither HashSet nor HashMap implement the Hash trait
    Array(Vec<NodeType>),
    Object {
        properties: HashMap<String, ObjectProperty>,
    },
    // TODO Use a HashSet. Problem: Neither HashSet nor HashMap implement the Hash trait
    Any(Vec<NodeType>),
}
