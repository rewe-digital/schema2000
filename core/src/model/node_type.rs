use crate::model::any::AnyNode;
use crate::model::array::ArrayNode;
use crate::model::date::DateNode;
use crate::model::datetime::DateTimeNode;
use crate::model::integer::IntegerNode;
use crate::model::number::NumberNode;
use crate::model::object::ObjectNode;
use crate::model::string::StringNode;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NodeType {
    Any(AnyNode),
    Array(ArrayNode),
    Boolean,
    Integer(IntegerNode),
    Null,
    Number(NumberNode),
    Object(ObjectNode),
    String(StringNode),
    DateTime(DateTimeNode),
    Date(DateNode),
}

impl NodeType {
    #[must_use]
    pub fn is_object(&self) -> bool {
        matches!(self, NodeType::Object { .. })
    }

    #[must_use]
    pub fn is_array(&self) -> bool {
        matches!(self, NodeType::Array(_))
    }
}

impl From<StringNode> for NodeType {
    fn from(s: StringNode) -> Self {
        NodeType::String(s)
    }
}

impl From<IntegerNode> for NodeType {
    fn from(i: IntegerNode) -> Self {
        NodeType::Integer(i)
    }
}

impl From<NumberNode> for NodeType {
    fn from(n: NumberNode) -> Self {
        NodeType::Number(n)
    }
}

impl From<ArrayNode> for NodeType {
    fn from(a: ArrayNode) -> Self {
        NodeType::Array(a)
    }
}

impl From<ObjectNode> for NodeType {
    fn from(o: ObjectNode) -> Self {
        NodeType::Object(o)
    }
}

impl From<AnyNode> for NodeType {
    fn from(a: AnyNode) -> Self {
        NodeType::Any(a)
    }
}

impl From<DateTimeNode> for NodeType {
    fn from(dt: DateTimeNode) -> Self {
        NodeType::DateTime(dt)
    }
}

impl From<DateNode> for NodeType {
    fn from(dt: DateNode) -> Self {
        NodeType::Date(dt)
    }
}
