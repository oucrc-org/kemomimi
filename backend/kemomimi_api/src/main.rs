use axum::{extract::Host, http::Method, Router};
use axum_extra::extract::CookieJar;
use chrono::Utc;
use kemomimi_api::app;
use openapi::server::new;
use sqlx::{PgPool, Pool, Postgres};
use std::env;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod libs;
use libs::ApiImpl;

mod routes;

#[tokio::main]
async fn main() {
    // initialize tracing

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {:?}", listener);
    axum::serve(listener, app().await).await.unwrap();
}
