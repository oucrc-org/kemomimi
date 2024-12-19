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
pub enum UsersGetResponse {
    /// 部員一覧
    Status200
    (Vec<models::User>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UsersPostResponse {
    /// 部員登録成功
    Status201
    (models::User)
    ,
    /// 必須項目の入力漏れなど
    Status400
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UsersUserIdDeleteResponse {
    /// 部員削除成功
    Status204
    ,
    /// 部員が見つかりません
    Status404
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UsersUserIdGetResponse {
    /// 部員情報
    Status200
    (models::User)
    ,
    /// 部員が見つかりません
    Status404
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UsersUserIdPutResponse {
    /// 部員情報更新成功
    Status200
    (models::User)
    ,
    /// 必須項目の入力漏れなど
    Status400
    ,
    /// 部員が見つかりません
    Status404
}


/// Users
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Users {
    /// 部員一覧取得.
    ///
    /// UsersGet - GET /users
    async fn users_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
    ) -> Result<UsersGetResponse, ()>;

    /// 部員新規登録.
    ///
    /// UsersPost - POST /users
    async fn users_post(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
            body: models::User,
    ) -> Result<UsersPostResponse, ()>;

    /// 部員削除.
    ///
    /// UsersUserIdDelete - DELETE /users-{user_id}
    async fn users_user_id_delete(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::UsersUserIdDeletePathParams,
    ) -> Result<UsersUserIdDeleteResponse, ()>;

    /// 部員情報取得.
    ///
    /// UsersUserIdGet - GET /users-{user_id}
    async fn users_user_id_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::UsersUserIdGetPathParams,
    ) -> Result<UsersUserIdGetResponse, ()>;

    /// 部員情報更新.
    ///
    /// UsersUserIdPut - PUT /users-{user_id}
    async fn users_user_id_put(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::UsersUserIdPutPathParams,
            body: models::User,
    ) -> Result<UsersUserIdPutResponse, ()>;
}
