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

#[derive(Clone, Debug)]
pub struct AppState {
    pub db_pool: Arc<Pool<Postgres>>,
}

impl AsRef<AppState> for AppState {
    fn as_ref(&self) -> &AppState {
        self
    }
}

pub async fn app() -> Router {
    // ログ収集の有効化
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    // dbと接続
    dotenvy::dotenv().expect("Failed to read .env file");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Arc::new(PgPool::connect(&database_url).await.unwrap());

    // app作成
    let state = AppState { db_pool: pool };
    new(state)
        .layer(CorsLayer::new().allow_origin(Any))
        .layer(TraceLayer::new_for_http())
}
