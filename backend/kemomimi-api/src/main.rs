use axum::{extract::Host, http::Method, Router};
use axum_extra::extract::CookieJar;
use chrono::Utc;
use openapi::server::new;
use sqlx::{PgPool, Pool, Postgres};
use std::env;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

mod libs;
use libs::ApiImpl;

mod routes;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to read .env file");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // initialize tracing
    tracing_subscriber::fmt::init();

    // TODO postgresへの移行
    let pool = Arc::new(PgPool::connect(&database_url).await.unwrap());
    // build our application with a route
    // ApiImpl を Arc に包む（必要なら）
    let api_impl = ApiImpl { db_pool: pool }; // 実際の構造体を定義する
    let router = new(api_impl).layer(CorsLayer::new().allow_origin(Any));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {:?}", listener);
    axum::serve(listener, router).await.unwrap();
}

impl AsRef<ApiImpl> for ApiImpl {
    fn as_ref(&self) -> &ApiImpl {
        self
    }
}
