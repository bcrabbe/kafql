// Integration test for kafql-store using testcontainers
use testcontainers::{clients, GenericImage};
use reqwest::blocking::Client;
use std::{thread, time::Duration};
use std::process::Command;

fn wait_for_kafka_ready(kafka_port: u16) {
    let addr = format!("localhost:{}", kafka_port);
    for _ in 0..30 {
        if std::net::TcpStream::connect(&addr).is_ok() {
            return;
        }
        thread::sleep(Duration::from_secs(1));
    }
    panic!("Kafka did not become ready in time");
}

fn print_container_logs(container: &testcontainers::Container<GenericImage>) {
    let id = container.id();
    let output = Command::new("docker")
        .args(&["logs", id])
        .output()
        .expect("failed to run docker logs");
    eprintln!("--- kafql-store logs ---");
    eprintln!("{}", String::from_utf8_lossy(&output.stdout));
    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    eprintln!("--- end logs ---");
}

fn wait_for_api_ready(api_port: u16) -> bool {
    let client = Client::new();
    let url = format!("http://localhost:{}/records", api_port);
    for _ in 0..30 {
        if let Ok(resp) = client.get(&url).send() {
            if resp.status().is_success() {
                return true;
            }
        }
        std::thread::sleep(Duration::from_secs(1));
    }
    false
}

#[test]
fn end_to_end_kafka_to_kafql() {
    let docker = clients::Cli::default();

    // Start Zookeeper
    let zookeeper = docker.run(
        GenericImage::new("bitnami/zookeeper", "3.8")
            .with_env_var("ALLOW_ANONYMOUS_LOGIN", "yes")
            .with_exposed_port(2181),
    );
    let zookeeper_host_port = zookeeper.get_host_port_ipv4(2181);

    // Start Kafka
    let kafka = docker.run(
        GenericImage::new("bitnami/kafka", "3.6")
            .with_env_var("KAFKA_BROKER_ID", "1")
            .with_env_var("KAFKA_ZOOKEEPER_CONNECT", format!("127.0.0.1:{}", zookeeper_host_port))
            .with_env_var("KAFKA_LISTENERS", "PLAINTEXT://:9092")
            .with_env_var("KAFKA_ADVERTISED_LISTENERS", "PLAINTEXT://localhost:9092")
            .with_env_var("ALLOW_PLAINTEXT_LISTENER", "yes")
            .with_exposed_port(9092),
    );
    let kafka_host_port = kafka.get_host_port_ipv4(9092);

    wait_for_kafka_ready(kafka_host_port);

    // Start kafql-store (assumes image is built and available as 'kafql-store:latest')
    let kafql = docker.run(
        GenericImage::new("kafql-store", "latest")
            .with_env_var("KAFKA_BROKERS", "kafka:9092")
            .with_env_var("KAFKA_TOPICS", "test:1")
            .with_env_var("HTTP_LISTEN", "0.0.0.0:3000")
            .with_exposed_port(3000),
    );
    let api_port = kafql.get_host_port_ipv4(3000);

    // Wait for API, print logs if not ready
    if !wait_for_api_ready(api_port) {
        print_container_logs(&kafql);
        panic!("kafql-store API did not become ready in time");
    }

    // Create the topic
    let _ = docker.run(
        GenericImage::new("bitnami/kafka", "3.6")
            .with_env_var("KAFKA_BROKER_ID", "1")
            .with_env_var("KAFKA_ZOOKEEPER_CONNECT", format!("127.0.0.1:{}", zookeeper_host_port))
            .with_env_var("KAFKA_LISTENERS", "PLAINTEXT://:9092")
            .with_env_var("KAFKA_ADVERTISED_LISTENERS", "PLAINTEXT://localhost:9092")
            .with_env_var("ALLOW_PLAINTEXT_LISTENER", "yes")
            .with_entrypoint(&format!(
                "/opt/bitnami/kafka/bin/kafka-topics.sh --create --if-not-exists --topic test --bootstrap-server host.docker.internal:{} --partitions 1 --replication-factor 1",
                kafka_host_port
            )),
    );

    // Produce a message to Kafka using a one-shot shell command
    let _ = docker.run(
        GenericImage::new("bitnami/kafka", "3.6")
            .with_env_var("KAFKA_BROKER_ID", "1")
            .with_env_var("KAFKA_ZOOKEEPER_CONNECT", format!("127.0.0.1:{}", zookeeper_host_port))
            .with_env_var("KAFKA_LISTENERS", "PLAINTEXT://:9092")
            .with_env_var("KAFKA_ADVERTISED_LISTENERS", "PLAINTEXT://localhost:9092")
            .with_env_var("ALLOW_PLAINTEXT_LISTENER", "yes")
            .with_entrypoint(&format!(
                "sh -c \"echo 'foo:{{\\\"hello\\\":\\\"world\\\"}}' | /opt/bitnami/kafka/bin/kafka-console-producer.sh --broker-list host.docker.internal:{} --topic test --property parse.key=true --property key.separator=:\"",
                kafka_host_port
            )),
    );
    thread::sleep(Duration::from_secs(3));

    // Query kafql-store API
    let client = Client::new();
    let resp = client.get(&format!("http://localhost:{}/records", api_port)).send().unwrap();
    assert!(resp.status().is_success());
    let records: serde_json::Value = resp.json().unwrap();
    assert!(records.as_array().unwrap().iter().any(|rec| rec["key"] == "foo" && rec["value"]["hello"] == "world"));
}
