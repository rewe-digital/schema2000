use backend::SchemaBuilder;
use renderer::render_debug;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let deserializer = serde_json::Deserializer::from_reader(stdin);
    let iterator = deserializer.into_iter::<serde_json::Value>();

    let mut sb = SchemaBuilder::new();

    for item in iterator {
        sb.inspect(&item?);
    }

    let result = render_debug(&sb);

    println!("{}", result);

    Ok(())
}
