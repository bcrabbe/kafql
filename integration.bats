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
           "echo 'foo:{\"hello\":\"world\"}' | \
           /opt/bitnami/kafka/bin/kafka-console-producer.sh \
           --broker-list kafka:9092 \
           --topic test \
           --property parse.key=true \
           --property key.separator=:"

    # Poll kafql-store API until the message appears or timeout after 10 seconds
    for i in {1..10}; do
        run curl -s http://localhost:3000/records
        if [[ "$output" == *'"key":"foo"'* ]]; then
            break
        fi
        sleep 1
    done

    echo "$output"
    [[ "$output" == *'"key":"foo"'* ]]
    [[ "$output" == *'"hello":"world"'* ]]
}
