use axum::{extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use openapi::{
    apis::private_items::{
        PrivateItems, PrivateItemsGetResponse, PrivateItemsPostResponse,
        PrivateItemsPrivateItemIdDeleteResponse, PrivateItemsPrivateItemIdGetResponse,
        PrivateItemsPrivateItemIdPutResponse,
    },
    models,
};

use crate::libs::ApiImpl;
impl PrivateItems for ApiImpl {
    #[doc = " 私物一覧取得."]
    #[doc = ""]
    #[doc = " PrivateItemsGet - GET /private-items"]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn private_items_get<'life0, 'async_trait>(
        &'life0 self,
        method: Method,
        host: Host,
        cookies: CookieJar,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<PrivateItemsGetResponse, ()>>
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

    #[doc = " 私物新規登録."]
    #[doc = ""]
    #[doc = " PrivateItemsPost - POST /private-items"]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn private_items_post<'life0, 'async_trait>(
        &'life0 self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::PrivateItem,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<PrivateItemsPostResponse, ()>>
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

    #[doc = " 私物削除."]
    #[doc = ""]
    #[doc = " PrivateItemsPrivateItemIdDelete - DELETE /private-items/{private-item-id}"]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn private_items_private_item_id_delete<'life0, 'async_trait>(
        &'life0 self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::PrivateItemsPrivateItemIdDeletePathParams,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<PrivateItemsPrivateItemIdDeleteResponse, ()>>
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

    #[doc = " 私物情報取得."]
    #[doc = ""]
    #[doc = " PrivateItemsPrivateItemIdGet - GET /private-items/{private-item-id}"]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn private_items_private_item_id_get<'life0, 'async_trait>(
        &'life0 self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::PrivateItemsPrivateItemIdGetPathParams,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<PrivateItemsPrivateItemIdGetResponse, ()>>
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

    #[doc = " 私物情報更新."]
    #[doc = ""]
    #[doc = " PrivateItemsPrivateItemIdPut - PUT /private-items/{private-item-id}"]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn private_items_private_item_id_put<'life0, 'async_trait>(
        &'life0 self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::PrivateItemsPrivateItemIdPutPathParams,
        body: models::PrivateItem,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<PrivateItemsPrivateItemIdPutResponse, ()>>
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
