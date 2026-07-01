use openapi::models;
use sqlx::{FromRow, PgPool};
use std::sync::Arc;

#[derive(Debug, Clone, FromRow)]
pub struct PrivateItemRow {
    pub private_item_id: String,
    pub name: String,
    pub owner_id: Option<String>,
    pub post_grad_treat_id: Option<String>,
    pub model_number: Option<String>,
    pub is_remaining: bool,
    pub remarks: Option<String>,
}

pub async fn list(pool: &Arc<PgPool>) -> Result<Vec<PrivateItemRow>, sqlx::Error> {
    sqlx::query_as::<_, PrivateItemRow>(
        r#"
        SELECT private_item_id, name, owner_id, post_grad_treat_id,
               model_number, is_remaining, remarks
        FROM private_item
        ORDER BY name
        "#,
    )
    .fetch_all(pool.as_ref())
    .await
}

pub async fn get(
    pool: &Arc<PgPool>,
    private_item_id: &str,
) -> Result<Option<PrivateItemRow>, sqlx::Error> {
    sqlx::query_as::<_, PrivateItemRow>(
        r#"
        SELECT private_item_id, name, owner_id, post_grad_treat_id,
               model_number, is_remaining, remarks
        FROM private_item
        WHERE private_item_id = $1
        "#,
    )
    .bind(private_item_id)
    .fetch_optional(pool.as_ref())
    .await
}

pub async fn insert(
    pool: &Arc<PgPool>,
    private_item_id: &str,
    name: &str,
    owner_id: Option<&str>,
    post_grad_treat_id: Option<&str>,
    model_number: Option<&str>,
    is_remaining: bool,
    remarks: Option<&str>,
) -> Result<PrivateItemRow, sqlx::Error> {
    sqlx::query_as::<_, PrivateItemRow>(
        r#"
        INSERT INTO private_item (
            private_item_id, name, owner_id, post_grad_treat_id,
            model_number, is_remaining, remarks
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING private_item_id, name, owner_id, post_grad_treat_id,
                  model_number, is_remaining, remarks
        "#,
    )
    .bind(private_item_id)
    .bind(name)
    .bind(owner_id)
    .bind(post_grad_treat_id)
    .bind(model_number)
    .bind(is_remaining)
    .bind(remarks)
    .fetch_one(pool.as_ref())
    .await
}

pub async fn update(
    pool: &Arc<PgPool>,
    private_item_id: &str,
    name: &str,
    owner_id: Option<&str>,
    post_grad_treat_id: Option<&str>,
    model_number: Option<&str>,
    is_remaining: bool,
    remarks: Option<&str>,
) -> Result<Option<PrivateItemRow>, sqlx::Error> {
    sqlx::query_as::<_, PrivateItemRow>(
        r#"
        UPDATE private_item
        SET name = $2, owner_id = $3, post_grad_treat_id = $4,
            model_number = $5, is_remaining = $6, remarks = $7
        WHERE private_item_id = $1
        RETURNING private_item_id, name, owner_id, post_grad_treat_id,
                  model_number, is_remaining, remarks
        "#,
    )
    .bind(private_item_id)
    .bind(name)
    .bind(owner_id)
    .bind(post_grad_treat_id)
    .bind(model_number)
    .bind(is_remaining)
    .bind(remarks)
    .fetch_optional(pool.as_ref())
    .await
}

pub async fn delete(pool: &Arc<PgPool>, private_item_id: &str) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM private_item WHERE private_item_id = $1")
        .bind(private_item_id)
        .execute(pool.as_ref())
        .await?;
    Ok(result.rows_affected() > 0)
}

pub fn to_model(row: PrivateItemRow) -> models::PrivateItem {
    models::PrivateItem {
        private_item_id: row.private_item_id,
        name: Some(row.name),
        owner_id: row.owner_id,
        post_grad_treat_id: row.post_grad_treat_id,
        model_number: row.model_number,
        is_remaining: Some(row.is_remaining),
        remarks: row.remarks,
    }
}
