mod common;

use axum::http::StatusCode;
use common::{delete, get, json_request, send, send_json, test_app};
use serde_json::{json, Value};
use sqlx::PgPool;

#[sqlx::test(migrations = "../migrations")]
async fn public_items_crud(pool: PgPool) -> sqlx::Result<()> {
    let mut app = test_app(pool);

    let (status, created): (StatusCode, Value) = send_json(
        &mut app,
        json_request(
            "POST",
            "/public-items",
            json!({
                "name": "テスト備品",
                "cost": 1000,
                "is_remaining": true,
                "purchase_request_id": "req-1"
            }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    let public_item_id = created["public_item_id"]
        .as_str()
        .expect("public_item_id should exist")
        .to_string();
    assert_eq!(created["name"], "テスト備品");

    let (status, items): (StatusCode, Vec<Value>) =
        send_json(&mut app, get("/public-items")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(items.len(), 1);

    let (status, item): (StatusCode, Value) = send_json(
        &mut app,
        get(&format!("/public-items/{public_item_id}")),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(item["public_item_id"], public_item_id);

    let (status, updated): (StatusCode, Value) = send_json(
        &mut app,
        json_request(
            "PUT",
            &format!("/public-items/{public_item_id}"),
            json!({
                "public_item_id": public_item_id,
                "name": "更新備品",
                "is_remaining": false
            }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(updated["name"], "更新備品");

    let (status, _) = send(
        &mut app,
        delete(&format!("/public-items/{public_item_id}")),
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let (status, _) = send(
        &mut app,
        get(&format!("/public-items/{public_item_id}")),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    Ok(())
}

#[sqlx::test(migrations = "../migrations")]
async fn public_items_search(pool: PgPool) -> sqlx::Result<()> {
    let mut app = test_app(pool);

    let (status, _): (StatusCode, Value) = send_json(
        &mut app,
        json_request(
            "POST",
            "/public-items",
            json!({
                "name": "kemomimi mouse",
                "purchase_request_id": "req-1"
            }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);

    let (status, items): (StatusCode, Vec<Value>) = send_json(
        &mut app,
        get("/public-items?search=mouse"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["name"], "kemomimi mouse");

    Ok(())
}
