use std::io::StdinLock;

use clap::Parser;
use serde_json::de::IoRead;
use serde_json::{StreamDeserializer, Value};

use schema2000::{generate_hypothesis_from_jsons, render_schema};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args = Args::parse();

    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let deserializer = serde_json::Deserializer::from_reader(stdin);
    let iterator: StreamDeserializer<IoRead<StdinLock>, Value> = deserializer.into_iter::<Value>();

    let hypothesises = generate_hypothesis_from_jsons(iterator.collect())?;

    hypothesises.values().for_each(|hypo| {
        let result = render_schema(hypo);
        println!("{}", result);
    });

    Ok(())
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {}
