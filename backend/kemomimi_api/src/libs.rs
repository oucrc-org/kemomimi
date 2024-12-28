use sqlx::{PgPool, Pool, Postgres};
use std::sync::Arc;
#[derive(Clone, Debug)]
pub struct ApiImpl {
    pub db_pool: Arc<Pool<Postgres>>,
}
