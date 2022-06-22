pub use any::AnyNode;
pub use array::ArrayNode;
pub use integer::IntegerNode;
pub use node_type::NodeType;
pub use number::NumberNode;
pub use object::{ObjectNode, ObjectProperty};
pub use string::StringNode;
pub use value_collection::ValueCollection;
pub use value_collection::MAX_VALUES;

mod any;
mod array;
mod integer;
mod node_type;
mod number;
mod object;
mod string;
mod value_collection;

#[derive(Debug, PartialEq)]
pub struct SchemaHypothesis {
    pub root: NodeType,
}

impl SchemaHypothesis {
    pub fn new<N: Into<NodeType>>(root: N) -> Self {
        SchemaHypothesis { root: root.into() }
    }
}
