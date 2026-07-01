use chrono::NaiveDate;
use openapi::models;
use sqlx::{FromRow, PgPool};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct PublicItemRow {
    pub public_item_id: Uuid,
    pub name: String,
    pub product_id: Uuid,
    pub cost: Option<i32>,
    pub purchase_date: NaiveDate,
    pub expiration_date: Option<NaiveDate>,
    pub is_remaining: bool,
    pub remarks: Option<String>,
    pub category_id: Option<String>,
    pub category_name: Option<String>,
    pub category_remarks: Option<String>,
}

pub struct ListParams<'a> {
    pub search: Option<&'a str>,
    pub sort: Option<&'a str>,
    pub filter: Option<&'a str>,
    pub filter_value: Option<&'a str>,
}

pub async fn list(
    pool: &Arc<PgPool>,
    params: ListParams<'_>,
) -> Result<Vec<PublicItemRow>, sqlx::Error> {
    let mut sql = String::from(
        r#"
        SELECT
            pi.public_item_id,
            pi.name,
            pi.product_id,
            pi.cost,
            pi.purchase_date,
            pi.expiration_date,
            pi.is_remaining,
            pi.remarks,
            c.category_id,
            c.name AS category_name,
            c.remarks AS category_remarks
        FROM public_item pi
        LEFT JOIN product p ON pi.product_id = p.product_id
        LEFT JOIN product_category pc ON p.product_id = pc.product_id
        LEFT JOIN category c ON pc.category_id = c.category_id
        WHERE 1 = 1
        "#,
    );

    if params.search.is_some() {
        sql.push_str(" AND pi.name ILIKE $1");
    }

    if params.filter == Some("is_remaining") {
        sql.push_str(" AND pi.is_remaining = TRUE");
    } else if params.filter == Some("category_id") {
        if params.filter_value.is_some() {
            sql.push_str(" AND c.category_id = $2");
        }
    }

    match params.sort {
        Some("cost") => sql.push_str(" ORDER BY pi.cost"),
        Some("approval_date") => sql.push_str(" ORDER BY pi.purchase_date"),
        Some("expiration_date") => sql.push_str(" ORDER BY pi.expiration_date"),
        _ => sql.push_str(" ORDER BY pi.public_item_id"),
    }

    let search_pattern = params
        .search
        .map(|value| format!("%{value}%"));

    let mut query = sqlx::query_as::<_, PublicItemRow>(&sql);
    if let Some(pattern) = search_pattern.as_deref() {
        query = query.bind(pattern);
    }
    if params.filter == Some("category_id") {
        if let Some(category_id) = params.filter_value {
            query = query.bind(category_id);
        }
    }

    query.fetch_all(pool.as_ref()).await
}

pub async fn get(
    pool: &Arc<PgPool>,
    public_item_id: Uuid,
) -> Result<Option<PublicItemRow>, sqlx::Error> {
    sqlx::query_as::<_, PublicItemRow>(
        r#"
        SELECT
            pi.public_item_id,
            pi.name,
            pi.product_id,
            pi.cost,
            pi.purchase_date,
            pi.expiration_date,
            pi.is_remaining,
            pi.remarks,
            c.category_id,
            c.name AS category_name,
            c.remarks AS category_remarks
        FROM public_item pi
        LEFT JOIN product p ON pi.product_id = p.product_id
        LEFT JOIN product_category pc ON p.product_id = pc.product_id
        LEFT JOIN category c ON pc.category_id = c.category_id
        WHERE pi.public_item_id = $1
        "#,
    )
    .bind(public_item_id)
    .fetch_optional(pool.as_ref())
    .await
}

pub async fn insert(
    pool: &Arc<PgPool>,
    public_item_id: Uuid,
    product_id: Uuid,
    name: &str,
    cost: Option<i32>,
    purchase_date: NaiveDate,
    expiration_date: Option<NaiveDate>,
    is_remaining: bool,
    remarks: Option<&str>,
) -> Result<PublicItemRow, sqlx::Error> {
    sqlx::query_as::<_, PublicItemRow>(
        r#"
        INSERT INTO public_item (
            public_item_id, name, product_id, cost, purchase_date,
            expiration_date, is_remaining, remarks
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING
            public_item_id,
            name,
            product_id,
            cost,
            purchase_date,
            expiration_date,
            is_remaining,
            remarks,
            NULL::TEXT AS category_id,
            NULL::TEXT AS category_name,
            NULL::TEXT AS category_remarks
        "#,
    )
    .bind(public_item_id)
    .bind(name)
    .bind(product_id)
    .bind(cost)
    .bind(purchase_date)
    .bind(expiration_date)
    .bind(is_remaining)
    .bind(remarks)
    .fetch_one(pool.as_ref())
    .await
}

pub async fn update(
    pool: &Arc<PgPool>,
    public_item_id: Uuid,
    name: Option<&str>,
    cost: Option<i32>,
    purchase_date: Option<NaiveDate>,
    expiration_date: Option<NaiveDate>,
    is_remaining: Option<bool>,
    remarks: Option<&str>,
) -> Result<Option<PublicItemRow>, sqlx::Error> {
    let current = get(pool, public_item_id).await?;
    let Some(current) = current else {
        return Ok(None);
    };

    sqlx::query_as::<_, PublicItemRow>(
        r#"
        UPDATE public_item
        SET
            name = $2,
            cost = $3,
            purchase_date = $4,
            expiration_date = $5,
            is_remaining = $6,
            remarks = $7
        WHERE public_item_id = $1
        RETURNING
            public_item_id,
            name,
            product_id,
            cost,
            purchase_date,
            expiration_date,
            is_remaining,
            remarks,
            NULL::TEXT AS category_id,
            NULL::TEXT AS category_name,
            NULL::TEXT AS category_remarks
        "#,
    )
    .bind(public_item_id)
    .bind(name.unwrap_or(&current.name))
    .bind(cost.or(current.cost))
    .bind(purchase_date.unwrap_or(current.purchase_date))
    .bind(expiration_date.or(current.expiration_date))
    .bind(is_remaining.unwrap_or(current.is_remaining))
    .bind(remarks.or(current.remarks.as_deref()))
    .fetch_optional(pool.as_ref())
    .await
}

pub async fn delete(pool: &Arc<PgPool>, public_item_id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM public_item WHERE public_item_id = $1")
        .bind(public_item_id)
        .execute(pool.as_ref())
        .await?;
    Ok(result.rows_affected() > 0)
}

pub fn to_public_item(row: &PublicItemRow) -> models::PublicItem {
    models::PublicItem {
        public_item_id: row.public_item_id.to_string(),
        name: row.name.clone(),
        category: row.category_id.as_ref().map(|category_id| models::Category {
            category_id: category_id.clone(),
            name: row.category_name.clone().unwrap_or_default(),
            remarks: row.category_remarks.clone(),
        }),
        cost: row.cost,
        approval_date: Some(row.purchase_date),
        expiration_date: row.expiration_date,
        is_remaining: row.is_remaining,
        main_user: None,
        remarks: row.remarks.clone(),
    }
}

pub async fn to_details(
    pool: &Arc<PgPool>,
    row: PublicItemRow,
    purchase_request_id: Option<String>,
) -> Result<models::PublicItemDetails, sqlx::Error> {
    let product_row = crate::db::products::get(pool, row.product_id)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;
    let product = crate::db::products::to_model(pool, product_row).await?;

    Ok(models::PublicItemDetails {
        public_item_id: row.public_item_id.to_string(),
        name: Some(row.name),
        product: Some(product),
        cost: row.cost,
        purchase_date: Some(row.purchase_date),
        expiration_date: row.expiration_date,
        is_remaining: Some(row.is_remaining),
        purchase_request_id,
        remarks: row.remarks,
    })
}
