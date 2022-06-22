use std::collections::HashMap;
use std::ops::Deref;

use clap::Parser;
use serde_json::Value;

use schema2000::{render_schema, SchemaHypothesis};

const MAGIC_KEY: &str = "hypothesis";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args = Args::parse();

    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let deserializer = serde_json::Deserializer::from_reader(stdin);
    let iterator = deserializer.into_iter::<Value>();

    let mut hypothesises: HashMap<String, SchemaHypothesis> = HashMap::new();

    for json_document in iterator {
        let document = &json_document?;
        let discriminator_key = &extract_discriminator(document);
        let new_hypo = schema2000::generate_hypothesis(extract_payload(document));

        if hypothesises.contains_key(discriminator_key) {
            let current = hypothesises.get(discriminator_key).unwrap().clone();
            let merged_hypo = schema2000::merge_hypothesis(current, new_hypo);
            hypothesises.insert(discriminator_key.to_string(), merged_hypo);
        } else {
            hypothesises.insert(discriminator_key.to_string(), new_hypo);
        }
    }

    hypothesises.iter().for_each(|(discriminator_key, hypo)| {
        let result = render_schema(hypo);
        println!("{}", result);
    });

    Ok(())
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

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {}
