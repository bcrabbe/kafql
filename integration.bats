#!/usr/bin/env bats

setup() {
    docker compose up -d zookeeper kafka
    # Wait for Kafka to be ready
    sleep 10
    # Create the test topic
    docker compose exec kafka /opt/bitnami/kafka/bin/kafka-topics.sh \
           --create --if-not-exists --topic test \
           --bootstrap-server kafka:9092 \
           --partitions 1 --replication-factor 1
    # Now start kafql-store
    docker compose up -d kafql-store
}

teardown() {
    docker compose down -v
}

@test "kafql-store end-to-end integration" {
    # Wait for kafql-store API to be ready
    for i in {1..30}; do
        echo "waiting for kafql-store"
        if curl -s http://localhost:3000/records | grep -q '\['; then
            echo "kafql-store ready"
            break
        fi
        sleep 2
    done

    # Produce a message to Kafka
    docker compose exec kafka bash -c \
           "echo 'foo:{\"hello\":\"world\"}' | /opt/bitnami/kafka/bin/kafka-console-producer.sh --broker-list kafka:9092 --topic test --property parse.key=true --property key.separator=:"

    # Wait for kafql-store to consume the message
    sleep 10

    # Query kafql-store API and check for the message
    run curl -s http://localhost:3000/records
    echo "$output"
    [[ "$output" == *'"key":"foo"'* ]]
    [[ "$output" == *'"hello":"world"'* ]]
}
