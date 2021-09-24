use crate::node_type::NodeType;
use crate::schema_node::SchemaNode;
use crate::simple_type::SimpleType;
use serde_json::Value;

/// The schema builder is the central entrypoint to build schemata
/// it holds all the configuration passed from a frontend/caller and is called repeatedly to update itself
#[derive(Default)]
pub struct SchemaBuilder {
    // currently only one root. To distinguish different messages/events ("created", "updated" etc.), we probably need multiple roots
    pub root: Option<SchemaNode>,
}

impl SchemaBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inspect(&mut self, value: &Value) {
        match &self.root {
            None => self.root = Some(SchemaNode::new(NodeType::Simple(SimpleType::new(value)))),
            Some(root) => {
                let next = SchemaNode::new(NodeType::Simple(SimpleType::new(value)));
                root.merge(next);
            }
        }
    }
}
