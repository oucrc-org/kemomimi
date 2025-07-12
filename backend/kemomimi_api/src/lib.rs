use axum::{
    body::Bytes,
    extract::{Host, MatchedPath},
    http::{HeaderMap, Method, Request},
    response::Response,
    Router,
};
use axum_extra::extract::CookieJar;
use chrono::Utc;
use openapi::server::new;
use sqlx::{PgPool, Pool, Postgres};
use std::sync::Arc;
use std::{env, time::Duration};
use tower_http::{
    classify::ServerErrorsFailureClass,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{info, info_span, Level, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod libs;
use libs::ApiImpl;

mod routes;

pub async fn app() -> Router {
    // ログ収集の有効化
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    dotenvy::dotenv().expect("Failed to read .env file");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = Arc::new(PgPool::connect(&database_url).await.unwrap());
    // build our application with a route
    // ApiImpl を Arc に包む（必要なら）
    let api_impl = ApiImpl { db_pool: pool }; // 実際の構造体を定義する
    new(api_impl)
        .layer(CorsLayer::new().allow_origin(Any))
        .layer(TraceLayer::new_for_http())
}

impl AsRef<ApiImpl> for ApiImpl {
    fn as_ref(&self) -> &ApiImpl {
        self
    }
}
