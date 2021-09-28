use serde_json::Value;
use std::iter::Map;

#[derive(Debug)]
pub struct SchemaHypothesis {
    root: NodeType,
}

#[derive(Debug)]
struct Node {
    node_type: NodeType,
    required: bool,
}

#[derive(Debug)]
enum NodeType {
    String,
    Integer,
    Number,
    Boolean,
    // Null,
    Array(Box<NodeType>), // ["a", "b", "c"] or [1,2,3,4]
    Object { properties: Map<String, Box<Node>> },
    // Enum/AnyOf
}

pub fn generate_hypothesis(_dom: &Value) -> SchemaHypothesis {
    SchemaHypothesis {root: NodeType::String}
}

pub fn merge_hypothesis(_a: SchemaHypothesis, _b: SchemaHypothesis) -> SchemaHypothesis {
    _a
}
