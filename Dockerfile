# Build stage
FROM rust:1.87-alpine AS builder
WORKDIR /usr/src/kafql-store
COPY ./kafql-store ./
RUN apk add --no-cache musl-dev cmake && \
    cargo build --release

# Runtime stage
FROM alpine:3.19
WORKDIR /app
COPY --from=builder /usr/src/kafql-store/target/release/kafql-store /app/kafql-store
COPY ./LICENSE /app/
EXPOSE 3000
ENTRYPOINT ["/app/kafql-store"]
CMD ["--help"]
