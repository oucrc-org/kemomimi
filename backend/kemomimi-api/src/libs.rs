use sqlx::{PgPool, Pool, Postgres};
use std::sync::Arc;
#[derive(Clone)]
pub struct ApiImpl {
    pub db_pool: Arc<Pool<Postgres>>,
}
