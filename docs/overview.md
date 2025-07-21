# kafql-store Overview

**kafql-store** is a lightweight, in-memory service that consumes messages from specified Kafka topics and partitions, stores them with log compaction and tombstoning, and exposes a REST API for CRUD operations on the data.

## Main Features
- Consumes from Kafka topics/partitions
- In-memory storage with log compaction and tombstoning
- REST API for CRUD (Create, Read, Update, Delete) operations
- JSON value support
- Easy configuration via CLI, environment variables, or Docker

## High-Level Architecture

```
Kafka Broker(s)
     │
     ▼
[kafql-store]
  ├─ Kafka Consumer (async)
  ├─ In-memory Store (compaction, tombstoning)
  └─ REST API (CRUD)
```

- The service connects to Kafka, consumes messages, and maintains the latest state in memory.
- The REST API allows external clients to query and modify the in-memory data.

## Use Cases
- Lightweight cache or query layer for compacted Kafka topics
- Prototyping or testing with ephemeral data
- Simple microservice for exposing Kafka topic state via HTTP 