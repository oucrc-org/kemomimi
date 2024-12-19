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
pub enum PublicItemsGetResponse {
    /// 備品一覧
    Status200
    (Vec<models::PublicItem>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PublicItemsPostResponse {
    /// 備品登録成功
    Status201
    (models::PublicItemDetails)
    ,
    /// 必須項目の入力漏れなど
    Status400
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PublicItemsPublicItemIdDeleteResponse {
    /// 備品削除成功
    Status204
    ,
    /// 備品が見つかりません
    Status404
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PublicItemsPublicItemIdGetResponse {
    /// 備品情報
    Status200
    (models::PublicItemDetails)
    ,
    /// 備品が見つかりません
    Status404
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PublicItemsPublicItemIdPutResponse {
    /// 備品情報更新成功
    Status200
    (models::PublicItemDetails)
    ,
    /// 必須項目の入力漏れなど
    Status400
    ,
    /// 備品が見つかりません
    Status404
}


/// PublicItems
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait PublicItems {
    /// 備品一覧取得.
    ///
    /// PublicItemsGet - GET /public-items
    async fn public_items_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      query_params: models::PublicItemsGetQueryParams,
    ) -> Result<PublicItemsGetResponse, ()>;

    /// 備品新規登録.
    ///
    /// PublicItemsPost - POST /public-items
    async fn public_items_post(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
            body: models::PublicItemEntry,
    ) -> Result<PublicItemsPostResponse, ()>;

    /// 備品削除.
    ///
    /// PublicItemsPublicItemIdDelete - DELETE /public-items/{public-item-id}
    async fn public_items_public_item_id_delete(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::PublicItemsPublicItemIdDeletePathParams,
    ) -> Result<PublicItemsPublicItemIdDeleteResponse, ()>;

    /// 備品情報取得.
    ///
    /// PublicItemsPublicItemIdGet - GET /public-items/{public-item-id}
    async fn public_items_public_item_id_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::PublicItemsPublicItemIdGetPathParams,
    ) -> Result<PublicItemsPublicItemIdGetResponse, ()>;

    /// 備品情報更新.
    ///
    /// PublicItemsPublicItemIdPut - PUT /public-items/{public-item-id}
    async fn public_items_public_item_id_put(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::PublicItemsPublicItemIdPutPathParams,
            body: models::PublicItemDetails,
    ) -> Result<PublicItemsPublicItemIdPutResponse, ()>;
}
