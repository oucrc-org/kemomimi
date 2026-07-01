mod common;

use axum::http::StatusCode;
use common::{delete, get, json_request, send, send_json, test_app};
use serde_json::{json, Value};
use sqlx::PgPool;

#[sqlx::test(migrations = "../migrations")]
async fn users_crud(pool: PgPool) -> sqlx::Result<()> {
    let mut app = test_app(pool);

    let (status, created): (StatusCode, Value) = send_json(
        &mut app,
        json_request(
            "POST",
            "/users",
            json!({
                "user_id": "user-1",
                "handle_name": "kemo",
                "screen_name": "けもみみ"
            }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(created["user_id"], "user-1");

    let (status, users): (StatusCode, Vec<Value>) = send_json(&mut app, get("/users")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(users.len(), 1);

    let (status, user): (StatusCode, Value) =
        send_json(&mut app, get("/users-user-1")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(user["screen_name"], "けもみみ");

    let (status, updated): (StatusCode, Value) = send_json(
        &mut app,
        json_request(
            "PUT",
            "/users-user-1",
            json!({
                "user_id": "user-1",
                "handle_name": "kemo",
                "screen_name": "更新後"
            }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(updated["screen_name"], "更新後");

    let (status, _) = send(&mut app, delete("/users-user-1")).await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let (status, _) = send(&mut app, get("/users-user-1")).await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    Ok(())
}
