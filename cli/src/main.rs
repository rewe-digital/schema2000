use clap::Parser;
use schema2000::{render_schema, SchemaHypothesis};
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let stdin: Box<dyn Read> = if let Some(file_path) = args.file {
        // Read from a file if the `--file` option is provided.
        let file = File::open(&file_path)?;
        Box::new(file)
    } else {
        // Read from standard input if the `--file` option is not provided.
        Box::new(io::stdin())
    };

    let deserializer = serde_json::Deserializer::from_reader(stdin);
    let iterator = deserializer.into_iter::<serde_json::Value>();

    let mut current_hypothesis: Option<SchemaHypothesis> = None;

    for json_document in iterator {
        let new_hypo = schema2000::generate_hypothesis(&json_document?);
        if current_hypothesis.is_none() {
            current_hypothesis = Some(new_hypo);
        } else {
            current_hypothesis =
                current_hypothesis.map(|cur| schema2000::merge_hypothesis(cur, new_hypo));
        }
    }

    let result = render_schema(&current_hypothesis.unwrap());

    println!("{}", result);

    Ok(())
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    /// JSON file path
    file: Option<String>,
}
