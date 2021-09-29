use backend::SchemaHypothesis;
use renderer::render_debug;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let deserializer = serde_json::Deserializer::from_reader(stdin);
    let iterator = deserializer.into_iter::<serde_json::Value>();

    //let mut sb = SchemaBuilder::new();

    let mut current_hypothesis: Option<SchemaHypothesis> = None;

    for json_document in iterator {
        let new_hypo = backend::generate_hypothesis(&json_document?);
        if current_hypothesis.is_none() {
            current_hypothesis = Some(new_hypo);
        } else {
            current_hypothesis =
                current_hypothesis.map(|cur| backend::merge_hypothesis(cur, new_hypo));
        }
    }

    let result = render_debug(&current_hypothesis.unwrap());

    println!("{}", result);

    Ok(())
}
