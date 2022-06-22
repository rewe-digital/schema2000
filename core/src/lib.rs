#![allow(clippy::module_name_repetitions)]

use serde_json::Value;
use std::collections::HashMap;

pub use generate::generate_hypothesis;
pub use merge::merge_hypothesis;
pub use model::SchemaHypothesis;
pub use renderer::render_schema;

mod generate;
mod merge;
pub mod model;
mod renderer;
mod utils;

pub fn generate_hypothesis_from_jsons(
    json_documents: Vec<serde_json::Result<Value>>,
) -> Result<HashMap<String, SchemaHypothesis>, Box<dyn std::error::Error>> {
    let mut hypothesises: HashMap<String, SchemaHypothesis> = HashMap::new();

    for json_document in json_documents {
        let document = &json_document?;
        let discriminator_key = &extract_discriminator(document);
        let new_hypo = generate_hypothesis(extract_payload(document));

        if hypothesises.contains_key(discriminator_key) {
            let current = hypothesises.get(discriminator_key).unwrap().clone();
            let merged_hypo = merge_hypothesis(current, new_hypo);
            hypothesises.insert(discriminator_key.to_string(), merged_hypo);
        } else {
            hypothesises.insert(discriminator_key.to_string(), new_hypo);
        }
    }
    Ok(hypothesises)
}

fn extract_payload(document: &Value) -> &Value {
    document.as_object().unwrap().get("payload").unwrap()
}

fn extract_discriminator(document: &Value) -> String {
    document
        .as_object()
        .unwrap()
        .get("discriminator")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod test {
    use crate::generate_hypothesis_from_jsons;
    use crate::model::NodeType;
    use crate::model::NodeType::Object;
    use maplit::hashset;
    use serde_json::Value;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn test_array_merge_objects() {
        let json_lines = "{ \"discriminator\": \"address\", \"payload\": { \"street\": \"Foo-Avenue\", \"number\": 15 } }\n\
            { \"discriminator\": \"address\", \"payload\": { \"street\": \"Central\", \"number\": 10 } }\n\
            { \"discriminator\": \"name\", \"payload\": { \"first_name\": \"First\", \"last_name\": \"Last\" } }\n\
            { \"discriminator\": \"name\", \"payload\": { \"first_name\": \"Last\", \"last_name\": 2 } }\n\
            { \"discriminator\": \"name\", \"payload\": { \"first_name\": \"Last\" } }";

        let deserializer = serde_json::Deserializer::from_str(json_lines);

        let json_documents = deserializer.into_iter::<Value>().collect();

        let actual = generate_hypothesis_from_jsons(json_documents).unwrap();

        assert_eq!(
            actual.keys().map(String::as_str).collect::<HashSet<&str>>(),
            hashset! {"address", "name"}
        );

        let address_toplevel_keys = get_toplevel_properties(&actual.get("address").unwrap().root);
        let name_toplevel_keys = get_toplevel_properties(&actual.get("name").unwrap().root);

        assert_eq!(address_toplevel_keys, hashset! {"number", "street"});
        assert_eq!(name_toplevel_keys, hashset! {"first_name", "last_name"});
    }

    fn get_toplevel_properties(root_node: &NodeType) -> HashSet<&str> {
        match root_node {
            Object(obj_node) => HashSet::from_iter(obj_node.properties.keys().map(String::as_str)),
            _ => unreachable!(),
        }
    }
}
