use sqlx::PgPool;
use std::sync::Arc;
#[derive(Clone)]
pub struct ApiImpl {
    pub db_pool: Arc<PgPool>,
}
