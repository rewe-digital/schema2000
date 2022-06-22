use std::error::Error;
use std::time::Duration;

use futures::StreamExt;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{Consumer, MessageStream};
use rdkafka::message::Message;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::Serialize;
use serde_json::Value;

use schema2000::{generate_hypothesis, render_json_schema, SchemaHypothesis};

const TOPIC_SCHEMA: &str = "schema2000";
const TOPIC_SCHEMA_LATEST: &str = "schema2000_latest";

#[derive(Clone, PartialEq)]
struct KafkaSchemaHypothesis {
    hypothesis: SchemaHypothesis,
    topic: String,
    offset: i64,
    partition: i32,
    key: Option<Vec<u8>>,
}

#[derive(Serialize)]
struct SchemaHypothesisMessage {
    schema: Value,
    topic: String,
    offset: i64,
    partition: i32,
    key: Option<Vec<u8>>,
}

impl From<KafkaSchemaHypothesis> for SchemaHypothesisMessage {
    fn from(source: KafkaSchemaHypothesis) -> Self {
        SchemaHypothesisMessage {
            schema: render_json_schema(&source.hypothesis),
            topic: source.topic.clone(),
            offset: source.offset,
            partition: source.partition,
            key: source.key.clone(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", "localhost")
        .set("group.id", "kcat")
        .set("enable.auto.commit", "false")
        .set("auto.offset.reset", "earliest")
        .create()
        .unwrap();

    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", "localhost")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    // get all topics
    let metadata = consumer.fetch_metadata(None, Duration::from_secs(60))?;
    let topics: Vec<&str> = metadata
        .topics()
        .iter()
        .map(|t| t.name())
        // remove our own topic, and 'internal' topics (starting with underscores)
        // TODO: is there an official documentation how to detect (kafka-internal) topics? This underscore-stuff ist just guessing here.
        .filter(|&t| t != TOPIC_SCHEMA && t != TOPIC_SCHEMA_LATEST && !t.starts_with("_"))
        .collect();

    // …and subscribe to all of them
    consumer.subscribe(&topics).unwrap();

    let stream: MessageStream = consumer.stream();

    // turn the stream of messages into a stream of schema hypothesis
    let mut schema_stream = stream.map(
        |m| -> Result<Option<KafkaSchemaHypothesis>, Box<dyn Error>> {
            let owned_message = m?.detach();
            let topic = owned_message.topic().to_string();
            let offset = owned_message.offset();
            let partition = owned_message.partition();
            let key = owned_message.key().map(|k| k.to_owned());

            let schema = owned_message
                .payload_view::<str>()
                .transpose()?
                .map(serde_json::from_str::<Value>)
                .transpose()?
                .as_ref()
                .map(generate_hypothesis);

            let schema = schema.map(|schema| KafkaSchemaHypothesis {
                hypothesis: schema,
                key,
                partition,
                topic,
                offset,
            });

            Ok(schema)
        },
    );

    let mut current_hypothesis_message: Option<KafkaSchemaHypothesis> = None;

    loop {
        let new = schema_stream.next().await.unwrap();

        // generate a new hypothesis
        let new_hypothesis = match (&current_hypothesis_message, new) {
            (None, Ok(h)) => h,
            (Some(cur), Ok(None)) => Some(cur.clone()),
            (Some(cur), Ok(Some(new))) => {
                let merged_hypothesis =
                    schema2000::merge_hypothesis(cur.to_owned().hypothesis, new.hypothesis);

                if merged_hypothesis != cur.hypothesis {
                    Some(KafkaSchemaHypothesis {
                        hypothesis: merged_hypothesis,
                        ..new
                    })
                } else {
                    Some(cur.clone())
                }
            }
            (cur, Err(e)) => {
                eprintln!("{:#?}", e);
                cur.clone()
            }
        };

        // schema2000
        // homeassistant_event_schema2000

        // if the merged hypothesis is a different one than the one we used to know, print it
        if new_hypothesis != current_hypothesis_message {
            current_hypothesis_message = new_hypothesis;
            let current_hypothesis_message = current_hypothesis_message.clone().unwrap();

            let message: SchemaHypothesisMessage = current_hypothesis_message.into();
            let payload: String = serde_json::to_string(&message).unwrap();

            let delivery_status_events = producer.send(
                FutureRecord::to(TOPIC_SCHEMA)
                    .payload(&payload)
                    .key(&*message.topic),
                //.headers(OwnedHeaders::new().add("header_key", "header_value")), // TODO: add the schema2000 version/commit as header
                Duration::from_secs(0),
            );

            let delivery_status_compact = producer.send(
                FutureRecord::to(TOPIC_SCHEMA_LATEST)
                    .payload(&payload)
                    .key(&*message.topic),
                //.headers(OwnedHeaders::new().add("header_key", "header_value")),
                Duration::from_secs(0),
            );

            let delivery_status_events = delivery_status_events.await;
            let delivery_status_compact = delivery_status_compact.await;

            // This will be executed when the result is received.
            println!(
                "{:?}, {:?}",
                delivery_status_events, delivery_status_compact
            );
        }
    }
}
