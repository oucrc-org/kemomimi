use crate::libs::ApiImpl;
use axum::{extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use openapi::{
    apis::users::{
        Users, UsersGetResponse, UsersPostResponse, UsersUserIdDeleteResponse,
        UsersUserIdGetResponse, UsersUserIdPutResponse,
    },
    models,
};
impl Users for ApiImpl {
    #[doc = " 部員一覧取得."]
    #[doc = ""]
    #[doc = " UsersGet - GET /users"]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn users_get<'life0, 'async_trait>(
        &'life0 self,
        method: Method,
        host: Host,
        cookies: CookieJar,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<UsersGetResponse, ()>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[doc = " 部員新規登録."]
    #[doc = ""]
    #[doc = " UsersPost - POST /users"]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn users_post<'life0, 'async_trait>(
        &'life0 self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::User,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<UsersPostResponse, ()>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[doc = " 部員削除."]
    #[doc = ""]
    #[doc = " UsersUserIdDelete - DELETE /users-{user_id}"]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn users_user_id_delete<'life0, 'async_trait>(
        &'life0 self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::UsersUserIdDeletePathParams,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<UsersUserIdDeleteResponse, ()>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[doc = " 部員情報取得."]
    #[doc = ""]
    #[doc = " UsersUserIdGet - GET /users-{user_id}"]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn users_user_id_get<'life0, 'async_trait>(
        &'life0 self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::UsersUserIdGetPathParams,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<UsersUserIdGetResponse, ()>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[doc = " 部員情報更新."]
    #[doc = ""]
    #[doc = " UsersUserIdPut - PUT /users-{user_id}"]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn users_user_id_put<'life0, 'async_trait>(
        &'life0 self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::UsersUserIdPutPathParams,
        body: models::User,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<UsersUserIdPutResponse, ()>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }
}
