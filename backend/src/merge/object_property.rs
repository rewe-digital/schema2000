use crate::{merge, ObjectProperty};

pub fn merge_object_property(
    a: Option<&ObjectProperty>,
    b: Option<&ObjectProperty>,
) -> ObjectProperty {
    match (a, b) {
        (Some(a), None) => ObjectProperty {
            required: false,
            ..a.clone()
        },
        (None, Some(b)) => ObjectProperty {
            required: false,
            ..b.clone()
        },
        (Some(a), Some(b)) => ObjectProperty {
            required: a.required && b.required,
            node_type: merge::merge_node_type(a.clone().node_type, b.clone().node_type),
        },
        (None, None) => unreachable!(),
    }
}
