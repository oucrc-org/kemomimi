use chrono::NaiveDate;
use openapi::models;
use sqlx::{FromRow, PgPool};
use std::sync::Arc;

#[derive(Debug, Clone, FromRow)]
pub struct UserRow {
    pub user_id: String,
    pub handle_name: String,
    pub screen_name: String,
    pub slack_id: Option<String>,
    pub is_admin: bool,
    pub is_member: bool,
    pub graduation_date: Option<NaiveDate>,
    pub remarks: Option<String>,
}

pub async fn list(pool: &Arc<PgPool>) -> Result<Vec<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        r#"
        SELECT user_id, handle_name, screen_name, slack_id,
               is_admin, is_member, graduation_date, remarks
        FROM app_user
        ORDER BY screen_name
        "#,
    )
    .fetch_all(pool.as_ref())
    .await
}

pub async fn get(pool: &Arc<PgPool>, user_id: &str) -> Result<Option<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        r#"
        SELECT user_id, handle_name, screen_name, slack_id,
               is_admin, is_member, graduation_date, remarks
        FROM app_user
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool.as_ref())
    .await
}

pub async fn insert(
    pool: &Arc<PgPool>,
    user_id: &str,
    handle_name: &str,
    screen_name: &str,
    slack_id: Option<&str>,
    is_admin: bool,
    is_member: bool,
    graduation_date: Option<NaiveDate>,
    remarks: Option<&str>,
) -> Result<UserRow, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        r#"
        INSERT INTO app_user (
            user_id, handle_name, screen_name, slack_id,
            is_admin, is_member, graduation_date, remarks
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING user_id, handle_name, screen_name, slack_id,
                  is_admin, is_member, graduation_date, remarks
        "#,
    )
    .bind(user_id)
    .bind(handle_name)
    .bind(screen_name)
    .bind(slack_id)
    .bind(is_admin)
    .bind(is_member)
    .bind(graduation_date)
    .bind(remarks)
    .fetch_one(pool.as_ref())
    .await
}

pub async fn update(
    pool: &Arc<PgPool>,
    user_id: &str,
    handle_name: &str,
    screen_name: &str,
    slack_id: Option<&str>,
    is_admin: bool,
    is_member: bool,
    graduation_date: Option<NaiveDate>,
    remarks: Option<&str>,
) -> Result<Option<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        r#"
        UPDATE app_user
        SET handle_name = $2, screen_name = $3, slack_id = $4,
            is_admin = $5, is_member = $6, graduation_date = $7, remarks = $8
        WHERE user_id = $1
        RETURNING user_id, handle_name, screen_name, slack_id,
                  is_admin, is_member, graduation_date, remarks
        "#,
    )
    .bind(user_id)
    .bind(handle_name)
    .bind(screen_name)
    .bind(slack_id)
    .bind(is_admin)
    .bind(is_member)
    .bind(graduation_date)
    .bind(remarks)
    .fetch_optional(pool.as_ref())
    .await
}

pub async fn delete(pool: &Arc<PgPool>, user_id: &str) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM app_user WHERE user_id = $1")
        .bind(user_id)
        .execute(pool.as_ref())
        .await?;
    Ok(result.rows_affected() > 0)
}

pub fn to_model(row: UserRow) -> models::User {
    models::User {
        user_id: row.user_id,
        handle_name: row.handle_name,
        screen_name: row.screen_name,
        slack_id: row.slack_id,
        is_admin: Some(row.is_admin),
        is_member: Some(row.is_member),
        graduation_date: row.graduation_date,
        remarks: row.remarks,
    }
}
