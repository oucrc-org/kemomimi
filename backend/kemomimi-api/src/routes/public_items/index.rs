use crate::libs::ApiImpl;
use axum::{extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use chrono::Utc;
use openapi::{
    apis::public_items::{
        PublicItems, PublicItemsGetResponse, PublicItemsPostResponse,
        PublicItemsPublicItemIdDeleteResponse, PublicItemsPublicItemIdGetResponse,
        PublicItemsPublicItemIdPutResponse,
    },
    models::{self, PublicItem},
};

impl PublicItems for ApiImpl {
    #[doc = " 備品一覧取得."]
    #[doc = ""]
    #[doc = " PublicItemsGet - GET /public-items"]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn public_items_get<'life0, 'async_trait>(
        &'life0 self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        query_params: models::PublicItemsGetQueryParams,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<PublicItemsGetResponse, ()>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            Ok(PublicItemsGetResponse::Status200(vec![PublicItem {
                public_item_id: "idid".to_string(),
                name: "KEMOMIMI".to_string(),
                category: None,
                cost: Some(1000),
                approval_date: Some(Utc::now().naive_local().date()),
                expiration_date: None,
                is_remaining: true,
                main_user: None,
                remarks: None,
            }]))
        })
    }

    #[doc = " 備品新規登録."]
    #[doc = ""]
    #[doc = " PublicItemsPost - POST /public-items"]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn public_items_post<'life0, 'async_trait>(
        &'life0 self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::PublicItemEntry,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<PublicItemsPostResponse, ()>>
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

    #[doc = " 備品削除."]
    #[doc = ""]
    #[doc = " PublicItemsPublicItemIdDelete - DELETE /public-items/{public-item-id}"]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn public_items_public_item_id_delete<'life0, 'async_trait>(
        &'life0 self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::PublicItemsPublicItemIdDeletePathParams,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<PublicItemsPublicItemIdDeleteResponse, ()>>
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

    #[doc = " 備品情報取得."]
    #[doc = ""]
    #[doc = " PublicItemsPublicItemIdGet - GET /public-items/{public-item-id}"]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn public_items_public_item_id_get<'life0, 'async_trait>(
        &'life0 self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::PublicItemsPublicItemIdGetPathParams,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<PublicItemsPublicItemIdGetResponse, ()>>
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

    #[doc = " 備品情報更新."]
    #[doc = ""]
    #[doc = " PublicItemsPublicItemIdPut - PUT /public-items/{public-item-id}"]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn public_items_public_item_id_put<'life0, 'async_trait>(
        &'life0 self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::PublicItemsPublicItemIdPutPathParams,
        body: models::PublicItemDetails,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<PublicItemsPublicItemIdPutResponse, ()>>
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
