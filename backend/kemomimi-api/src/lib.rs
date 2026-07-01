pub mod db;
pub mod libs;
mod routes;

use axum::Router;
use libs::ApiImpl;
use openapi::server::new;
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

pub fn build_app(db_pool: Arc<PgPool>) -> Router {
    let api_impl = ApiImpl { db_pool };
    new(api_impl).layer(CorsLayer::new().allow_origin(Any))
}

impl AsRef<ApiImpl> for ApiImpl {
    fn as_ref(&self) -> &ApiImpl {
        self
    }
}
