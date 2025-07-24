default:
    just build

build:
    cargo build --release --manifest-path kafql-store/Cargo.toml

docker-build:
    docker build -t kafql-store .

run:
    cargo run \
    --release \
    --manifest-path kafql-store/Cargo.toml

start-local:
    ./start-local.sh

stop-local:
    docker compose down -v

test:
    cargo test --manifest-path kafql-store/Cargo.toml -- --test-threads=1

integration-test:
    bats integration.bats

fmt:
    cargo fmt --all
