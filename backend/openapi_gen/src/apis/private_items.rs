use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PrivateItemsGetResponse {
    /// 私物一覧
    Status200
    (Vec<models::PrivateItem>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PrivateItemsPostResponse {
    /// 私物登録成功
    Status201
    (models::PrivateItem)
    ,
    /// 必須項目の入力漏れなど
    Status400
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PrivateItemsPrivateItemIdDeleteResponse {
    /// 私物削除成功
    Status204
    ,
    /// 私物が見つかりません
    Status404
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PrivateItemsPrivateItemIdGetResponse {
    /// 私物情報
    Status200
    (models::PrivateItem)
    ,
    /// 私物が見つかりません
    Status404
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PrivateItemsPrivateItemIdPutResponse {
    /// 私物情報更新成功
    Status200
    (models::PrivateItem)
    ,
    /// 必須項目の入力漏れなど
    Status400
    ,
    /// 私物が見つかりません
    Status404
}


/// PrivateItems
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait PrivateItems {
    /// 私物一覧取得.
    ///
    /// PrivateItemsGet - GET /private-items
    async fn private_items_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
    ) -> Result<PrivateItemsGetResponse, ()>;

    /// 私物新規登録.
    ///
    /// PrivateItemsPost - POST /private-items
    async fn private_items_post(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
            body: models::PrivateItem,
    ) -> Result<PrivateItemsPostResponse, ()>;

    /// 私物削除.
    ///
    /// PrivateItemsPrivateItemIdDelete - DELETE /private-items/{private-item-id}
    async fn private_items_private_item_id_delete(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::PrivateItemsPrivateItemIdDeletePathParams,
    ) -> Result<PrivateItemsPrivateItemIdDeleteResponse, ()>;

    /// 私物情報取得.
    ///
    /// PrivateItemsPrivateItemIdGet - GET /private-items/{private-item-id}
    async fn private_items_private_item_id_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::PrivateItemsPrivateItemIdGetPathParams,
    ) -> Result<PrivateItemsPrivateItemIdGetResponse, ()>;

    /// 私物情報更新.
    ///
    /// PrivateItemsPrivateItemIdPut - PUT /private-items/{private-item-id}
    async fn private_items_private_item_id_put(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::PrivateItemsPrivateItemIdPutPathParams,
            body: models::PrivateItem,
    ) -> Result<PrivateItemsPrivateItemIdPutResponse, ()>;
}
