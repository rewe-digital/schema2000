use std::collections::HashMap;
use std::ops::Deref;

use clap::Parser;

use schema2000::{render_schema, SchemaHypothesis};

const MAGIC_KEY: String = "hypothesis".to_string();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args = Args::parse();

    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let deserializer = serde_json::Deserializer::from_reader(stdin);
    let iterator = deserializer.into_iter::<serde_json::Value>();

    let mut hypothesises: HashMap<String, SchemaHypothesis> = HashMap::new();

    for json_document in iterator {
        let new_hypo = schema2000::generate_hypothesis(&json_document?);

        if hypothesises.contains_key(&MAGIC_KEY) {
            let current = hypothesises.get(&MAGIC_KEY).unwrap().clone();
            let merged_hypo = schema2000::merge_hypothesis(current, new_hypo);
            hypothesises.insert(MAGIC_KEY, merged_hypo);
        } else {
            hypothesises.insert(MAGIC_KEY, new_hypo);
        }
    }

    let result = render_schema(hypothesises.get(&MAGIC_KEY).unwrap());

    println!("{}", result);

    Ok(())
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {}
