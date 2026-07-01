use kemomimi_api::build_app;
use sqlx::PgPool;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to read .env file");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    tracing_subscriber::fmt::init();

    let pool = Arc::new(PgPool::connect(&database_url).await.unwrap());
    let router = build_app(pool);

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {:?}", listener);
    axum::serve(listener, router).await.unwrap();
}
