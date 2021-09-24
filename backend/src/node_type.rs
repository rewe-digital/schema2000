use crate::schema_node::SchemaNode;
use crate::simple_type::SimpleType;

/// https://json-schema.org/understanding-json-schema/reference/combining.html
#[derive(Debug)]
pub enum NodeType {
    AllOf(Vec<SchemaNode>), // and
    AnyOf(Vec<SchemaNode>), // or
    OneOf(Vec<SchemaNode>), // xor
    Not(SchemaNode),        // not

    Enum(Vec<SimpleType>),
    Simple(SimpleType),
}
