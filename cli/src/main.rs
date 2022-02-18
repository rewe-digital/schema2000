use clap::Command;
use schema2000::{render_schema, SchemaHypothesis};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    parse_arguments();

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

fn parse_arguments() {
    Command::new("Schema2000")
        .version(env!("CARGO_PKG_VERSION"))
        .author(AUTHORS)
        .about("Reads a line seperated list of json files from the stdin and generates the JSON Schema which is written to the stdout")
        .get_matches();
}

const AUTHORS: &str = "Created and maintained by:
    Elmar Athmer <elmar.athmer@rewe-digital.com>,
    Lukas Prediger <lukas.prediger@rewe-digital.com>,
    Stefan Scheidt <stefan.scheidt@rewe-ditial.com,
    Christoph Baudson,
    Mischa Dieterle,
    Patrick Thäle";
