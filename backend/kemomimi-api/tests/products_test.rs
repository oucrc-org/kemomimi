mod common;

use axum::http::StatusCode;
use common::{delete, get, json_request, send, send_json, test_app};
use serde_json::{json, Value};
use sqlx::PgPool;

const PRODUCT_ID: &str = "550e8400-e29b-41d4-a716-446655440000";

#[sqlx::test(migrations = "../migrations")]
async fn products_list_starts_empty(pool: PgPool) -> sqlx::Result<()> {
    let mut app = test_app(pool);
    let (status, products): (StatusCode, Vec<Value>) = send_json(&mut app, get("/products")).await;
    assert_eq!(status, StatusCode::OK);
    assert!(products.is_empty());
    Ok(())
}

#[sqlx::test(migrations = "../migrations")]
async fn products_crud(pool: PgPool) -> sqlx::Result<()> {
    let mut app = test_app(pool);

    let (status, created): (StatusCode, Value) = send_json(
        &mut app,
        json_request(
            "POST",
            "/products",
            json!({
                "product_id": PRODUCT_ID,
                "name": "テスト製品",
                "model_number": "TM-001"
            }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(created["name"], "テスト製品");

    let (status, products): (StatusCode, Vec<Value>) =
        send_json(&mut app, get("/products")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(products.len(), 1);

    let (status, product): (StatusCode, Value) =
        send_json(&mut app, get(&format!("/products/{PRODUCT_ID}"))).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(product["product_id"], PRODUCT_ID);

    let (status, updated): (StatusCode, Value) = send_json(
        &mut app,
        json_request(
            "PUT",
            &format!("/products/{PRODUCT_ID}"),
            json!({
                "product_id": PRODUCT_ID,
                "name": "更新後製品"
            }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(updated["name"], "更新後製品");

    let (status, _) = send(&mut app, delete(&format!("/products/{PRODUCT_ID}"))).await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let (status, _) = send(&mut app, get(&format!("/products/{PRODUCT_ID}"))).await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    Ok(())
}

#[sqlx::test(migrations = "../migrations")]
async fn products_post_rejects_empty_name(pool: PgPool) -> sqlx::Result<()> {
    let mut app = test_app(pool);
    let (status, _) = send(
        &mut app,
        json_request(
            "POST",
            "/products",
            json!({
                "product_id": PRODUCT_ID,
                "name": ""
            }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    Ok(())
}
