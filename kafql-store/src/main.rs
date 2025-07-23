use anyhow::Result;
use clap::Parser;
use std::collections::HashMap;

mod api;
mod consumer;
mod store;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Config {
    /// Kafka broker list (comma-separated)
    #[arg(long, env = "KAFKA_BROKERS")]
    kafka_brokers: String,

    /// Consumer group ID
    #[arg(long, env = "KAFKA_GROUP_ID", default_value = "kafql-store")]
    group_id: String,

    /// Topic configurations in format: topic1:partitions,topic2:partitions
    #[arg(long, env = "KAFKA_TOPICS")]
    topics: String,

    /// HTTP API listen address
    #[arg(long, env = "HTTP_LISTEN", default_value = "0.0.0.0:3000")]
    http_listen: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Parse configuration
    let config = Config::parse();

    // Parse topics configuration
    let topics: HashMap<String, i32> = config
        .topics
        .split(',')
        .filter_map(|s| {
            let parts: Vec<&str> = s.split(':').collect();
            if parts.len() == 2 {
                Some((parts[0].to_string(), parts[1].parse().unwrap_or(1)))
            } else {
                None
            }
        })
        .collect();

    // Initialize the store
    let store = store::Store::new();

    // Start the consumer service
    let consumer_handle = {
        let store = store.clone();
        tokio::spawn(async move {
            consumer::run_consumer(config.kafka_brokers, config.group_id, topics, store).await
        })
    };

    // Start the API server
    let api_handle = {
        let store = store.clone();
        tokio::spawn(async move { api::run_server(config.http_listen, store).await })
    };

    // Wait for both services
    tokio::try_join!(async { consumer_handle.await? }, async {
        api_handle.await?
    })?;

    Ok(())
}
