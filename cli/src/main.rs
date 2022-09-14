use clap::Parser;
use schema2000::{generate_hypothesis_from_iterator, render_schema, SchemaHypothesis};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args = Args::parse();

    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let deserializer = serde_json::Deserializer::from_reader(stdin);
    let iterator = deserializer.into_iter::<serde_json::Value>();

    let hypothesis: SchemaHypothesis =
        generate_hypothesis_from_iterator(iterator.map(|value| value.unwrap()));

    let result = render_schema(&hypothesis);

    println!("{}", result);

    Ok(())
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {}
