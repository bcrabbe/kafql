# Usage Guide: kafql-store

## Running kafql-store

### With Cargo
```sh
cargo run --release -- \
  --kafka-brokers localhost:9092 \
  --topics "topic1:1,topic2:3" \
  --http-listen "0.0.0.0:3000"
```

### With Docker
```sh
docker build -t kafql-store .
docker run --rm -p 3000:3000 \
  -e KAFKA_BROKERS=localhost:9092 \
  -e KAFKA_TOPICS=test:1 \
  -e HTTP_LISTEN=0.0.0.0:3000 \
  kafql-store
```

## Configuration
You can configure kafql-store via command-line flags or environment variables:

- `--kafka-brokers` / `KAFKA_BROKERS`: Comma-separated list of Kafka brokers
- `--topics` / `KAFKA_TOPICS`: List of topics and partition counts, e.g. `topic1:1,topic2:3`
- `--http-listen` / `HTTP_LISTEN`: Address for the HTTP API server (default: `0.0.0.0:3000`)

## REST API Basics
See [api.md](api.md) for full details.

- `GET /records` — List all records
- `GET /records/:key` — Get a record by key
- `POST /records` — Create or update a record
- `DELETE /records/:key` — Delete a record (tombstone) 