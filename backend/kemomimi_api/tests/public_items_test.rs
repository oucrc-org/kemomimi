use axum::{
    body::{self, Body},
    http::{header, Request, StatusCode},
};
use tower::util::ServiceExt;

use kemomimi_api::app;
use tracing::info;

#[sqlx::test]
fn test_public_items() {
    let app = app().await;

    let req = Request::builder()
        .method("GET")
        .header(header::HOST, "localhost:3030")
        .uri("http://localhost:3030/public-items?")
        .body(Body::empty())
        .unwrap(); // buildの結果のエラーはunwrapで処理

    // リクエストを送信
    let resp = app.oneshot(req).await.unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    info!("this is resp {:?}", resp);
    let body = body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let body = String::from_utf8_lossy(&body);
    info!("body: {}", body);
    // let pb1 = PublicItemEntry {
    //     name: "KEMOMIMI".to_string(),
    //     cost: Some(100),
    //     purchase_date: None,
    //     expiration_date: None,
    //     is_remaining: None,
    //     purchase_request_id: "00001".to_string(),
    //     remarks: None,
    // };
    // let json_string = to_string(&pb1).unwrap();
    // let req = Request::builder()
    //     .method("POST")
    //     .uri("/public-items")
    //     .header(header::CONTENT_TYPE, "application/json")
    //     .body(axum::body::Body::from(json_string))
    //     .unwrap(); // buildの結果のエラーはunwrapで処理

    // // リクエストを送信
    // let resp = app.borrow_mut().oneshot(req).await.unwrap();

    // // レスポンスステータスコードの検証
    // assert_eq!(resp.status(), StatusCode::CREATED);
}
