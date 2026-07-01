mod common;

use axum::http::StatusCode;
use common::{delete, get, json_request, send, send_json, test_app};
use serde_json::{json, Value};
use sqlx::PgPool;

#[sqlx::test(migrations = "../migrations")]
async fn categories_crud(pool: PgPool) -> sqlx::Result<()> {
    let mut app = test_app(pool);

    let (status, created): (StatusCode, Value) = send_json(
        &mut app,
        json_request(
            "POST",
            "/categories",
            json!({
                "category_id": "cat-1",
                "name": "カテゴリ1"
            }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(created["category_id"], "cat-1");

    let (status, categories): (StatusCode, Vec<Value>) =
        send_json(&mut app, get("/categories")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(categories.len(), 1);

    let (status, category): (StatusCode, Value) =
        send_json(&mut app, get("/categories/cat-1")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(category["name"], "カテゴリ1");

    let (status, updated): (StatusCode, Value) = send_json(
        &mut app,
        json_request(
            "PUT",
            "/categories/cat-1",
            json!({
                "category_id": "cat-1",
                "name": "更新カテゴリ"
            }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(updated["name"], "更新カテゴリ");

    let (status, _) = send(&mut app, delete("/categories/cat-1")).await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let (status, _) = send(&mut app, get("/categories/cat-1")).await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    Ok(())
}
