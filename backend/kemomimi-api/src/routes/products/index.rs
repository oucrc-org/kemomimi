use crate::libs::ApiImpl;
use axum::{extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use openapi::{
    apis::products::{
        Products, ProductsGetResponse, ProductsPostResponse, ProductsProductIdDeleteResponse,
        ProductsProductIdGetResponse, ProductsProductIdPutResponse,
    },
    models,
};
impl Products for ApiImpl {
    #[doc = " 製品一覧取得."]
    #[doc = ""]
    #[doc = " ProductsGet - GET /products"]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn products_get<'life0, 'async_trait>(
        &'life0 self,
        method: Method,
        host: Host,
        cookies: CookieJar,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<ProductsGetResponse, ()>>
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

    #[doc = " 製品新規登録."]
    #[doc = ""]
    #[doc = " ProductsPost - POST /products"]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn products_post<'life0, 'async_trait>(
        &'life0 self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::Product,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<ProductsPostResponse, ()>>
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

    #[doc = " 製品削除."]
    #[doc = ""]
    #[doc = " ProductsProductIdDelete - DELETE /products/{product-id}"]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn products_product_id_delete<'life0, 'async_trait>(
        &'life0 self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ProductsProductIdDeletePathParams,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<ProductsProductIdDeleteResponse, ()>>
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

    #[doc = " 製品情報取得."]
    #[doc = ""]
    #[doc = " ProductsProductIdGet - GET /products/{product-id}"]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn products_product_id_get<'life0, 'async_trait>(
        &'life0 self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ProductsProductIdGetPathParams,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<ProductsProductIdGetResponse, ()>>
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

    #[doc = " 製品情報更新."]
    #[doc = ""]
    #[doc = " ProductsProductIdPut - PUT /products/{product-id}"]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn products_product_id_put<'life0, 'async_trait>(
        &'life0 self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ProductsProductIdPutPathParams,
        body: models::Product,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<ProductsProductIdPutResponse, ()>>
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
