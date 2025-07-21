default:
    just build

build:
    cargo build --release --manifest-path kafql-store/Cargo.toml

run:
    cargo run --release --manifest-path kafql-store/Cargo.toml -- --kafka-brokers localhost:9092 --topics "test:1" --http-listen "0.0.0.0:3000"

test:
    cargo test --manifest-path kafql-store/Cargo.toml

fmt:
    cargo fmt --all
