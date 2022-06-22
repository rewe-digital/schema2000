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
    use crate::model::NodeType::Object;
    use crate::model::ObjectNode;
    use maplit::hashmap;
    use serde_json::{json, Value};

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
            actual.keys().collect::<Vec<&String>>(),
            vec!("address", "name")
        );

        let top_level_keys = actual
            .iter()
            .map(|(discriminant, hypo)| match &hypo.root {
                Object(obj_node) => (
                    discriminant,
                    obj_node.properties.keys().collect::<Vec<&String>>(),
                ),
                _ => unreachable!(),
            })
            .collect();

        assert_eq!(top_level_keys.get("address"), vec!("number", "street"));
        assert_eq!(top_level_keys.get("name"), vec!("first_name", "last_name"));

        println!("{:?}", top_level_keys)
        // let expected = hashmap! {
        //     "address" => ObjectNode { properties:  }
        // }
        // assert_eq!(actual, expected);
    }
}
