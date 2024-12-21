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
pub enum ProductsGetResponse {
    /// 製品一覧
    Status200
    (Vec<models::Product>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ProductsPostResponse {
    /// 製品登録成功
    Status201
    (models::Product)
    ,
    /// 必須項目の入力漏れなど
    Status400
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ProductsProductIdDeleteResponse {
    /// 製品削除成功
    Status204
    ,
    /// 製品が見つかりません
    Status404
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ProductsProductIdGetResponse {
    /// 製品情報
    Status200
    (models::Product)
    ,
    /// 製品が見つかりません
    Status404
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ProductsProductIdPutResponse {
    /// 製品情報更新成功
    Status200
    (models::Product)
    ,
    /// 必須項目の入力漏れなど
    Status400
    ,
    /// 製品が見つかりません
    Status404
}


/// Products
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Products {
    /// 製品一覧取得.
    ///
    /// ProductsGet - GET /products
    async fn products_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
    ) -> Result<ProductsGetResponse, ()>;

    /// 製品新規登録.
    ///
    /// ProductsPost - POST /products
    async fn products_post(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
            body: models::Product,
    ) -> Result<ProductsPostResponse, ()>;

    /// 製品削除.
    ///
    /// ProductsProductIdDelete - DELETE /products/{product-id}
    async fn products_product_id_delete(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::ProductsProductIdDeletePathParams,
    ) -> Result<ProductsProductIdDeleteResponse, ()>;

    /// 製品情報取得.
    ///
    /// ProductsProductIdGet - GET /products/{product-id}
    async fn products_product_id_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::ProductsProductIdGetPathParams,
    ) -> Result<ProductsProductIdGetResponse, ()>;

    /// 製品情報更新.
    ///
    /// ProductsProductIdPut - PUT /products/{product-id}
    async fn products_product_id_put(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::ProductsProductIdPutPathParams,
            body: models::Product,
    ) -> Result<ProductsProductIdPutResponse, ()>;
}
