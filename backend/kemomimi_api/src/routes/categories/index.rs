use crate::libs::ApiImpl;
use axum::{extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use openapi::{
    apis::categories::{
        Categories, CategoriesCategoryIdDeleteResponse, CategoriesCategoryIdGetResponse,
        CategoriesCategoryIdPutResponse, CategoriesGetResponse, CategoriesPostResponse,
    },
    models,
};
impl Categories for ApiImpl {
    #[doc = " カテゴリ削除."]
    #[doc = ""]
    #[doc = " CategoriesCategoryIdDelete - DELETE /categories/{category-id}"]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn categories_category_id_delete<'life0, 'async_trait>(
        &'life0 self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::CategoriesCategoryIdDeletePathParams,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<CategoriesCategoryIdDeleteResponse, ()>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        let _ = method;
        todo!()
    }

    #[doc = " カテゴリ情報取得."]
    #[doc = ""]
    #[doc = " CategoriesCategoryIdGet - GET /categories/{category-id}"]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn categories_category_id_get<'life0, 'async_trait>(
        &'life0 self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::CategoriesCategoryIdGetPathParams,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<CategoriesCategoryIdGetResponse, ()>>
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

    #[doc = " カテゴリ情報更新."]
    #[doc = ""]
    #[doc = " CategoriesCategoryIdPut - PUT /categories/{category-id}"]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn categories_category_id_put<'life0, 'async_trait>(
        &'life0 self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::CategoriesCategoryIdPutPathParams,
        body: models::Category,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<CategoriesCategoryIdPutResponse, ()>>
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

    #[doc = " カテゴリ一覧取得."]
    #[doc = ""]
    #[doc = " CategoriesGet - GET /categories"]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn categories_get<'life0, 'async_trait>(
        &'life0 self,
        method: Method,
        host: Host,
        cookies: CookieJar,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<CategoriesGetResponse, ()>>
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

    #[doc = " カテゴリ新規登録."]
    #[doc = ""]
    #[doc = " CategoriesPost - POST /categories"]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn categories_post<'life0, 'async_trait>(
        &'life0 self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::Category,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<CategoriesPostResponse, ()>>
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
