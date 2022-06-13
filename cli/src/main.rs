use clap::Parser;
use schema2000::{render_schema, SchemaHypothesis};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args = Args::parse();

    let stdin = std::io::stdin();
    let stdin = stdin.lock();

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
struct Args {}
