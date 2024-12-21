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
pub enum CategoriesCategoryIdDeleteResponse {
    /// カテゴリ削除成功
    Status204
    ,
    /// カテゴリが見つかりません
    Status404
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CategoriesCategoryIdGetResponse {
    /// カテゴリ情報
    Status200
    (models::Category)
    ,
    /// カテゴリが見つかりません
    Status404
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CategoriesCategoryIdPutResponse {
    /// カテゴリ情報更新成功
    Status200
    (models::Category)
    ,
    /// 必須項目の入力漏れなど
    Status400
    ,
    /// カテゴリが見つかりません
    Status404
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CategoriesGetResponse {
    /// カテゴリ一覧
    Status200
    (Vec<models::Category>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CategoriesPostResponse {
    /// カテゴリ登録成功
    Status201
    (models::Category)
    ,
    /// 必須項目の入力漏れなど
    Status400
}


/// Categories
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Categories {
    /// カテゴリ削除.
    ///
    /// CategoriesCategoryIdDelete - DELETE /categories/{category-id}
    async fn categories_category_id_delete(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::CategoriesCategoryIdDeletePathParams,
    ) -> Result<CategoriesCategoryIdDeleteResponse, ()>;

    /// カテゴリ情報取得.
    ///
    /// CategoriesCategoryIdGet - GET /categories/{category-id}
    async fn categories_category_id_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::CategoriesCategoryIdGetPathParams,
    ) -> Result<CategoriesCategoryIdGetResponse, ()>;

    /// カテゴリ情報更新.
    ///
    /// CategoriesCategoryIdPut - PUT /categories/{category-id}
    async fn categories_category_id_put(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::CategoriesCategoryIdPutPathParams,
            body: models::Category,
    ) -> Result<CategoriesCategoryIdPutResponse, ()>;

    /// カテゴリ一覧取得.
    ///
    /// CategoriesGet - GET /categories
    async fn categories_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
    ) -> Result<CategoriesGetResponse, ()>;

    /// カテゴリ新規登録.
    ///
    /// CategoriesPost - POST /categories
    async fn categories_post(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
            body: models::Category,
    ) -> Result<CategoriesPostResponse, ()>;
}
