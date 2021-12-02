use futures::StreamExt;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{Consumer, MessageStream};
use rdkafka::message::Message;
use serde_json::Value;
use std::error::Error;

use schema2000::{generate_hypothesis, SchemaHypothesis};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", "localhost")
        .set("group.id", "kcat")
        .set("enable.auto.commit", "false")
        .create()
        .unwrap();

    consumer.subscribe(&["homeassistant_event"]).unwrap();

    let stream: MessageStream = consumer.stream();

    // turn the stream of messages into a stream of schema hypothesis
    let mut schema_stream = stream.map(|m| -> Result<Option<SchemaHypothesis>, Box<dyn Error>> {
        let schema = m?
            .payload_view::<str>()
            .transpose()?
            .map(serde_json::from_str::<Value>)
            .transpose()?
            .as_ref()
            .map(generate_hypothesis);

        Ok(schema)
    });

    let mut current_hypothesis: Option<SchemaHypothesis> = None;

    loop {
        let new = schema_stream.next().await.unwrap();

        // generate a new hypothesis
        let new_hypothesis = match (current_hypothesis.clone(), new) {
            (None, Ok(h)) => h,
            (Some(cur), Ok(None)) => Some(cur),
            (Some(cur), Ok(Some(new))) => Some(schema2000::merge_hypothesis(cur, new)),
            (cur, Err(e)) => {
                eprintln!("{:#?}", e);
                cur
            }
        };

        // if the merged hypothesis is a different one than the one we used to know, print it
        if new_hypothesis != current_hypothesis {
            current_hypothesis = new_hypothesis;
            println!(
                "{}",
                schema2000::render_schema(current_hypothesis.as_ref().unwrap())
            );
        }
    }
}
