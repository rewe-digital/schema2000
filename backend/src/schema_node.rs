use crate::node_type::NodeType;
use std::borrow::Borrow;

/// a schema node is a basic property in
#[derive(Debug)]
pub struct SchemaNode {
    node_type: Box<NodeType>,
}

impl SchemaNode {
    pub fn new(node_type: NodeType) -> Self {
        SchemaNode {
            node_type: Box::new(node_type),
        }
    }

    pub(crate) fn merge(&self, _other: SchemaNode) {
        match self.node_type.borrow() {
            NodeType::AllOf(_) => {}
            NodeType::AnyOf(_) => {}
            NodeType::OneOf(_) => {}
            NodeType::Not(_) => {}
            NodeType::Simple(_simple) => {
                // what could happen here?
                // `other` perfectly holds the criterias of `simple` -> nothing to do
                // `other` is of the same type as `simple` (e.g. numeric), but simple requires extension (i.e. maxLength/minLength)
                // `other` is of another type than simple, e.g. "string" and "null" -> enum (Enum not yet modelled)

                // whatever: assign the newly generated node_type to self.node_type
            }
        }
    }
}
