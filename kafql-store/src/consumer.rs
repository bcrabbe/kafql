use crate::store::{Record, Store};
use anyhow::Result;
use rdkafka::{
    client::ClientContext,
    config::{ClientConfig, RDKafkaLogLevel},
    consumer::{Consumer, ConsumerContext, Rebalance, StreamConsumer},
    error::KafkaResult,
    message::Message,
    topic_partition_list::TopicPartitionList,
    util::get_rdkafka_version,
};
use std::collections::HashMap;
use tracing::{error, info, warn};

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        info!("Pre rebalance: {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        info!("Post rebalance: {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        match result {
            Ok(_) => info!("Offsets committed successfully"),
            Err(e) => warn!("Error while committing offsets: {}", e),
        }
    }
}

pub async fn run_consumer(
    brokers: String,
    group_id: String,
    topics: HashMap<String, i32>,
    store: Store,
) -> Result<()> {
    info!(
        "Starting Kafka consumer with librdkafka v{:?}",
        get_rdkafka_version()
    );

    let context = CustomContext;

    let consumer: StreamConsumer<CustomContext> = ClientConfig::new()
        .set("group.id", &group_id)
        .set("bootstrap.servers", &brokers)
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "earliest")
        .set("enable.partition.eof", "false")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)?;

    let topic_list: Vec<String> = topics.keys().cloned().collect();
    let topic_refs: Vec<&str> = topic_list.iter().map(|s| s.as_str()).collect();
    consumer.subscribe(&topic_refs)?;

    info!("Subscribed to topics: {:?}", topic_list);

    loop {
        match consumer.recv().await {
            Ok(msg) => {
                let key = match msg.key() {
                    Some(k) => String::from_utf8_lossy(k).to_string(),
                    None => {
                        warn!("Received message without key, skipping");
                        continue;
                    }
                };

                let value = msg.payload().map(|p| {
                    serde_json::from_slice(p).unwrap_or_else(|_| {
                        serde_json::Value::String(String::from_utf8_lossy(p).to_string())
                    })
                });

                let record = Record {
                    key,
                    value,
                    timestamp: msg.timestamp().to_millis().unwrap_or(0) as u64,
                    topic: msg.topic().to_string(),
                    partition: msg.partition(),
                    offset: msg.offset(),
                };

                store.upsert(record);
            }
            Err(e) => {
                error!("Kafka error: {}", e);
            }
        }
    }
}
