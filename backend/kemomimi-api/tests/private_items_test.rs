mod common;

use axum::http::StatusCode;
use common::{delete, get, json_request, send, send_json, test_app};
use serde_json::{json, Value};
use sqlx::PgPool;

#[sqlx::test(migrations = "../migrations")]
async fn private_items_crud(pool: PgPool) -> sqlx::Result<()> {
    let mut app = test_app(pool);

    let (status, created): (StatusCode, Value) = send_json(
        &mut app,
        json_request(
            "POST",
            "/private-items",
            json!({
                "private_item_id": "priv-1",
                "name": "私物PC",
                "is_remaining": true
            }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(created["private_item_id"], "priv-1");

    let (status, items): (StatusCode, Vec<Value>) =
        send_json(&mut app, get("/private-items")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(items.len(), 1);

    let (status, item): (StatusCode, Value) =
        send_json(&mut app, get("/private-items/priv-1")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(item["name"], "私物PC");

    let (status, updated): (StatusCode, Value) = send_json(
        &mut app,
        json_request(
            "PUT",
            "/private-items/priv-1",
            json!({
                "private_item_id": "priv-1",
                "name": "更新後PC",
                "is_remaining": false
            }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(updated["name"], "更新後PC");

    let (status, _) = send(&mut app, delete("/private-items/priv-1")).await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let (status, _) = send(&mut app, get("/private-items/priv-1")).await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    Ok(())
}
