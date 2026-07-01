use chrono::NaiveDate;
use openapi::models;
use sqlx::{FromRow, PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct ProductRow {
    pub product_id: Uuid,
    pub name: String,
    pub model_number: Option<String>,
    pub product_url: Option<String>,
    pub remarks: Option<String>,
}

pub async fn list(pool: &Arc<PgPool>) -> Result<Vec<ProductRow>, sqlx::Error> {
    sqlx::query_as::<_, ProductRow>(
        r#"
        SELECT product_id, name, model_number, product_url, remarks
        FROM product
        ORDER BY name
        "#,
    )
    .fetch_all(pool.as_ref())
    .await
}

pub async fn get(pool: &Arc<PgPool>, product_id: Uuid) -> Result<Option<ProductRow>, sqlx::Error> {
    sqlx::query_as::<_, ProductRow>(
        r#"
        SELECT product_id, name, model_number, product_url, remarks
        FROM product
        WHERE product_id = $1
        "#,
    )
    .bind(product_id)
    .fetch_optional(pool.as_ref())
    .await
}

pub async fn insert(
    pool: &Arc<PgPool>,
    product_id: Uuid,
    name: &str,
    model_number: Option<&str>,
    product_url: Option<&str>,
    remarks: Option<&str>,
) -> Result<ProductRow, sqlx::Error> {
    sqlx::query_as::<_, ProductRow>(
        r#"
        INSERT INTO product (product_id, name, model_number, product_url, remarks)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING product_id, name, model_number, product_url, remarks
        "#,
    )
    .bind(product_id)
    .bind(name)
    .bind(model_number)
    .bind(product_url)
    .bind(remarks)
    .fetch_one(pool.as_ref())
    .await
}

pub async fn update(
    pool: &Arc<PgPool>,
    product_id: Uuid,
    name: &str,
    model_number: Option<&str>,
    product_url: Option<&str>,
    remarks: Option<&str>,
) -> Result<Option<ProductRow>, sqlx::Error> {
    sqlx::query_as::<_, ProductRow>(
        r#"
        UPDATE product
        SET name = $2, model_number = $3, product_url = $4, remarks = $5
        WHERE product_id = $1
        RETURNING product_id, name, model_number, product_url, remarks
        "#,
    )
    .bind(product_id)
    .bind(name)
    .bind(model_number)
    .bind(product_url)
    .bind(remarks)
    .fetch_optional(pool.as_ref())
    .await
}

pub async fn delete(pool: &Arc<PgPool>, product_id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM product WHERE product_id = $1")
        .bind(product_id)
        .execute(pool.as_ref())
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn categories_for_product(
    pool: &Arc<PgPool>,
    product_id: Uuid,
) -> Result<Vec<models::Category>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT c.category_id, c.name, c.remarks
        FROM category c
        INNER JOIN product_category pc ON c.category_id = pc.category_id
        WHERE pc.product_id = $1
        "#,
    )
    .bind(product_id)
    .fetch_all(pool.as_ref())
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| models::Category {
            category_id: row.get("category_id"),
            name: row.get("name"),
            remarks: row.get("remarks"),
        })
        .collect())
}

pub async fn users_for_product(
    pool: &Arc<PgPool>,
    product_id: Uuid,
) -> Result<Vec<models::User>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT u.user_id, u.handle_name, u.screen_name, u.slack_id,
               u.is_admin, u.is_member, u.graduation_date, u.remarks
        FROM app_user u
        INNER JOIN product_user pu ON u.user_id = pu.user_id
        WHERE pu.product_id = $1
        "#,
    )
    .bind(product_id)
    .fetch_all(pool.as_ref())
    .await?;

    Ok(rows.into_iter().map(user_from_row).collect())
}

pub async fn set_categories(
    pool: &Arc<PgPool>,
    product_id: Uuid,
    category_ids: &[String],
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM product_category WHERE product_id = $1")
        .bind(product_id)
        .execute(&mut tx)
        .await?;
    for category_id in category_ids {
        sqlx::query(
            "INSERT INTO product_category (product_id, category_id) VALUES ($1, $2)",
        )
        .bind(product_id)
        .bind(category_id)
        .execute(&mut tx)
        .await?;
    }
    tx.commit().await
}

pub async fn set_main_users(
    pool: &Arc<PgPool>,
    product_id: Uuid,
    user_ids: &[String],
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM product_user WHERE product_id = $1")
        .bind(product_id)
        .execute(&mut tx)
        .await?;
    for user_id in user_ids {
        sqlx::query("INSERT INTO product_user (product_id, user_id) VALUES ($1, $2)")
            .bind(product_id)
            .bind(user_id)
            .execute(&mut tx)
            .await?;
    }
    tx.commit().await
}

pub async fn find_or_create_by_name(pool: &Arc<PgPool>, name: &str) -> Result<Uuid, sqlx::Error> {
    if let Some(row) = sqlx::query("SELECT product_id FROM product WHERE name = $1 LIMIT 1")
        .bind(name)
        .fetch_optional(pool.as_ref())
        .await?
    {
        return Ok(row.get("product_id"));
    }

    let product_id = Uuid::new_v4();
    insert(pool, product_id, name, None, None, None).await?;
    Ok(product_id)
}

fn user_from_row(row: sqlx::postgres::PgRow) -> models::User {
    models::User {
        user_id: row.get("user_id"),
        handle_name: row.get("handle_name"),
        screen_name: row.get("screen_name"),
        slack_id: row.get("slack_id"),
        is_admin: Some(row.get("is_admin")),
        is_member: Some(row.get("is_member")),
        graduation_date: row.get::<Option<NaiveDate>, _>("graduation_date"),
        remarks: row.get("remarks"),
    }
}

pub async fn to_model(pool: &Arc<PgPool>, row: ProductRow) -> Result<models::Product, sqlx::Error> {
    let categories = categories_for_product(pool, row.product_id).await?;
    let main_users = users_for_product(pool, row.product_id).await?;
    Ok(models::Product {
        product_id: row.product_id.to_string(),
        name: row.name,
        model_number: row.model_number,
        product_url: row.product_url,
        categiries: if categories.is_empty() {
            None
        } else {
            Some(categories)
        },
        main_users: if main_users.is_empty() {
            None
        } else {
            Some(main_users)
        },
        remarks: row.remarks,
    })
}
