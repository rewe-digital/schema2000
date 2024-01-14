use clap::Parser;
use schema2000::{generate_hypothesis_from_iterator, render_schema, SchemaHypothesis};
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let reader: Box<dyn Read> = get_reader(args.file);

    let deserializer = serde_json::Deserializer::from_reader(reader);
    let iterator = deserializer.into_iter::<serde_json::Value>();

    let hypothesis: SchemaHypothesis =
        generate_hypothesis_from_iterator(iterator.map(|value| value.unwrap()));

    let result = render_schema(&hypothesis);

    println!("{result}");

    Ok(())
}

fn get_reader(path: Option<String>) -> Box<dyn Read> {
    if let Some(file_path) = path {
        // Read from a file if the `--file` option is provided.
        let file = File::open(file_path).expect("Failed to open file");
        Box::new(file)
    } else {
        // Read from standard input if the `--file` option is not provided.
        Box::new(io::stdin())
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    /// JSON file path
    file: Option<String>,
}
