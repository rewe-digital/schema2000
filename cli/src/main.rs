use std::io::StdinLock;

use clap::Parser;
use serde_json::de::IoRead;
use serde_json::{StreamDeserializer, Value};

use schema2000::{generate_hypothesis_from_jsons, render_schemas};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args = Args::parse();

    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let deserializer = serde_json::Deserializer::from_reader(stdin);
    let iterator: StreamDeserializer<IoRead<StdinLock>, Value> = deserializer.into_iter::<Value>();

    // TODO make use_discriminator configurable before merging, and set it by default to false in order to stay backward compatible
    let hypothesises = generate_hypothesis_from_jsons(iterator.collect(), true)?;

    let result = render_schemas(hypothesises);
    println!("{}", result);

    Ok(())
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {}
