# kafql-store

**kafql-store** is a simple tool that consumes messages from specified Kafka topics and partitions, stores them in memory (with log compaction and tombstoning), and serves a REST API for CRUD operations on the data.

## Features
- Consumes from Kafka topics/partitions
- In-memory storage with log compaction and tombstoning
- REST API for CRUD (Create, Read, Update, Delete) operations
- JSON value support
- Easy configuration via CLI or environment variables

## Usage

### Running

```sh
cargo run --release -- \
  --kafka-brokers localhost:9092 \
  --topics "topic1:1,topic2:3" \
  --http-listen "0.0.0.0:3000"
```

### Configuration
- `--kafka-brokers`: Comma-separated list of Kafka brokers
- `--topics`: List of topics and partition counts, e.g. `topic1:1,topic2:3`
- `--http-listen`: Address for the HTTP API server (default: `0.0.0.0:3000`)

### REST API
- `GET /records` — List all records
- `GET /records/:key` — Get a record by key
- `POST /records` — Create or update a record
- `DELETE /records/:key` — Delete a record (tombstone)

#### Example Record JSON
```json
{
  "key": "user-123",
  "value": {"name": "Alice"},
  "topic": "users",
  "partition": 0
}
```

## License

This project is licensed under the AGPL-3.0-or-later. See LICENSE for details.
