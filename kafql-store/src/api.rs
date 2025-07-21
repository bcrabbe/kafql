use crate::store::{Record, Store};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    serve, Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
struct CreateRequest {
    key: String,
    value: serde_json::Value,
    topic: String,
    partition: i32,
}

pub async fn run_server(addr: String, store: Store) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/records", get(list_records))
        .route("/records/:key", get(get_record))
        .route("/records", post(create_record))
        .route("/records/:key", delete(delete_record))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::new().allow_origin(Any))
        .with_state(store);

    let addr: SocketAddr = addr.parse()?;
    info!("Starting API server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    serve(listener, app).await?;

    Ok(())
}

async fn list_records(State(store): State<Store>) -> impl IntoResponse {
    let records = store.list();
    Json(records)
}

async fn get_record(State(store): State<Store>, Path(key): Path<String>) -> impl IntoResponse {
    match store.get(&key) {
        Some(record) => Ok(Json(record)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_record(
    State(store): State<Store>,
    Json(payload): Json<CreateRequest>,
) -> impl IntoResponse {
    let record = Record {
        key: payload.key,
        value: Some(payload.value),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
        topic: payload.topic,
        partition: payload.partition,
        offset: -1, // Since this is created via API
    };

    store.upsert(record.clone());
    (StatusCode::CREATED, Json(record))
}

async fn delete_record(State(store): State<Store>, Path(key): Path<String>) -> impl IntoResponse {
    match store.get(&key) {
        Some(record) => {
            let tombstone = store.create_tombstone(&key, &record.topic, record.partition, -1);
            store.upsert(tombstone);
            Ok(StatusCode::NO_CONTENT)
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}
