use crate::merge::object_property;
use crate::model::ObjectNode;
use std::collections::HashSet;

pub fn merge_object(a: ObjectNode, b: ObjectNode) -> ObjectNode {
    let properties_a = a.properties;
    let properties_b = b.properties;

    let keys_a: HashSet<&String> = properties_a.keys().collect();
    let keys_b: HashSet<&String> = properties_b.keys().collect();
    let merged_properties = keys_a
        .union(&keys_b)
        .map(|key| {
            (
                (*key).to_string(),
                object_property::merge_object_property(
                    properties_a.get(*key),
                    properties_b.get(*key),
                ),
            )
        })
        .collect();

    ObjectNode {
        properties: merged_properties,
    }
}
