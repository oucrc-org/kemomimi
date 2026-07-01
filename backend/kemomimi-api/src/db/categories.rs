use openapi::models;
use sqlx::{FromRow, PgPool};
use std::sync::Arc;

#[derive(Debug, Clone, FromRow)]
pub struct CategoryRow {
    pub category_id: String,
    pub name: String,
    pub remarks: Option<String>,
}

pub async fn list(pool: &Arc<PgPool>) -> Result<Vec<CategoryRow>, sqlx::Error> {
    sqlx::query_as::<_, CategoryRow>(
        r#"
        SELECT category_id, name, remarks
        FROM category
        ORDER BY name
        "#,
    )
    .fetch_all(pool.as_ref())
    .await
}

pub async fn get(
    pool: &Arc<PgPool>,
    category_id: &str,
) -> Result<Option<CategoryRow>, sqlx::Error> {
    sqlx::query_as::<_, CategoryRow>(
        r#"
        SELECT category_id, name, remarks
        FROM category
        WHERE category_id = $1
        "#,
    )
    .bind(category_id)
    .fetch_optional(pool.as_ref())
    .await
}

pub async fn insert(
    pool: &Arc<PgPool>,
    category_id: &str,
    name: &str,
    remarks: Option<&str>,
) -> Result<CategoryRow, sqlx::Error> {
    sqlx::query_as::<_, CategoryRow>(
        r#"
        INSERT INTO category (category_id, name, remarks)
        VALUES ($1, $2, $3)
        RETURNING category_id, name, remarks
        "#,
    )
    .bind(category_id)
    .bind(name)
    .bind(remarks)
    .fetch_one(pool.as_ref())
    .await
}

pub async fn update(
    pool: &Arc<PgPool>,
    category_id: &str,
    name: &str,
    remarks: Option<&str>,
) -> Result<Option<CategoryRow>, sqlx::Error> {
    sqlx::query_as::<_, CategoryRow>(
        r#"
        UPDATE category
        SET name = $2, remarks = $3
        WHERE category_id = $1
        RETURNING category_id, name, remarks
        "#,
    )
    .bind(category_id)
    .bind(name)
    .bind(remarks)
    .fetch_optional(pool.as_ref())
    .await
}

pub async fn delete(pool: &Arc<PgPool>, category_id: &str) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM category WHERE category_id = $1")
        .bind(category_id)
        .execute(pool.as_ref())
        .await?;
    Ok(result.rows_affected() > 0)
}

pub fn to_model(row: CategoryRow) -> models::Category {
    models::Category {
        category_id: row.category_id,
        name: row.name,
        remarks: row.remarks,
    }
}
