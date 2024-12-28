use std::borrow::BorrowMut;

use axum::{
    body::Body,
    http::{header, Request, StatusCode},
};
use futures_channel::oneshot;
use kemomimi_api::{self, app};
use serde_json::{json, to_string};
use sqlx::PgPool;
use tower::ServiceExt;

use openapi::{
    self,
    models::{PublicItem, PublicItemEntry},
};
// fn test_public_items(db: PgPool) {
#[sqlx::test]
fn test_public_items() {
    let mut app = app().await;

    let req = Request::builder()
        .method("GET")
        .uri("http://localhost:3030/public-items")
        .body(Body::empty())
        .unwrap(); // buildの結果のエラーはunwrapで処理

    // リクエストを送信
    let resp = app.oneshot(req).await.unwrap();

    println!("{:?}", resp);
    assert_eq!(resp.status(), StatusCode::OK);

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
