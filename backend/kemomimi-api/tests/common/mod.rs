use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use http_body_util::BodyExt;
use kemomimi_api::build_app;
use serde::de::DeserializeOwned;
use sqlx::PgPool;
use std::sync::Arc;
use tower::ServiceExt;

pub fn test_app(pool: PgPool) -> Router {
    build_app(Arc::new(pool))
}

pub async fn send(app: &mut Router, request: Request<Body>) -> (StatusCode, Vec<u8>) {
    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    (status, bytes.to_vec())
}

pub async fn send_json<T: DeserializeOwned>(
    app: &mut Router,
    request: Request<Body>,
) -> (StatusCode, T) {
    let (status, body) = send(app, request).await;
    let parsed = serde_json::from_slice(&body).unwrap_or_else(|error| {
        panic!(
            "failed to parse json: {error}\nbody: {}",
            String::from_utf8_lossy(&body)
        );
    });
    (status, parsed)
}

pub fn json_request(method: &str, uri: &str, body: impl serde::Serialize) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header("host", "localhost")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&body).unwrap()))
        .unwrap()
}

pub fn get(uri: &str) -> Request<Body> {
    Request::builder()
        .uri(uri)
        .header("host", "localhost")
        .body(Body::empty())
        .unwrap()
}

pub fn delete(uri: &str) -> Request<Body> {
    Request::builder()
        .method("DELETE")
        .uri(uri)
        .header("host", "localhost")
        .body(Body::empty())
        .unwrap()
}
