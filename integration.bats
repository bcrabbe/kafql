#!/usr/bin/env bats

setup_file() {
    just start-local
}

teardown_file() {
    just stop-local
}

@test "produce and fetch" {
    # Produce a message to Kafka
    docker compose exec kafka bash -c \
           "echo 'foo:{\"hello\":\"world\"}' | /opt/bitnami/kafka/bin/kafka-console-producer.sh --broker-list kafka:9092 --topic test --property parse.key=true --property key.separator=:"
    # Wait for kafql-store to consume the message
    sleep 2
    # Query kafql-store API and check for the message
    run curl -s http://localhost:3000/records
    echo "$output"
    [[ "$output" == *'"key":"foo"'* ]]
    [[ "$output" == *'"hello":"world"'* ]]

    # tombstone a message via Kafka
    docker compose exec kafka bash -c \
           "echo 'foo:' | /opt/bitnami/kafka/bin/kafka-console-producer.sh --broker-list kafka:9092 --topic test --property parse.key=true --property key.separator=:"
    # Wait for kafql-store to consume the message
    sleep 2
    run curl -s http://localhost:3000/records
    echo "$output"
    [[ "$output" == *'[]'* ]]
}
