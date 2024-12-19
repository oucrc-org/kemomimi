use axum::{extract::Host, http::Method, Router};
use axum_extra::extract::CookieJar;
use chrono::Utc;
use openapi::{
    apis::{
        categories::{
            Categories, CategoriesCategoryIdDeleteResponse, CategoriesCategoryIdGetResponse,
            CategoriesCategoryIdPutResponse, CategoriesGetResponse, CategoriesPostResponse,
        },
        private_items::{
            PrivateItems, PrivateItemsGetResponse, PrivateItemsPostResponse,
            PrivateItemsPrivateItemIdDeleteResponse, PrivateItemsPrivateItemIdGetResponse,
            PrivateItemsPrivateItemIdPutResponse,
        },
        products::{
            Products, ProductsGetResponse, ProductsPostResponse, ProductsProductIdDeleteResponse,
            ProductsProductIdGetResponse, ProductsProductIdPutResponse,
        },
        public_items::{
            PublicItems, PublicItemsGetResponse, PublicItemsPostResponse,
            PublicItemsPublicItemIdDeleteResponse, PublicItemsPublicItemIdGetResponse,
            PublicItemsPublicItemIdPutResponse,
        },
        users::{
            Users, UsersGetResponse, UsersPostResponse, UsersUserIdDeleteResponse,
            UsersUserIdGetResponse, UsersUserIdPutResponse,
        },
    },
    models::{self, PublicItem},
    server::new,
};
use sqlx::{PgPool, Pool, Postgres};
use std::env;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct ApiImpl {
    db_pool: Arc<Pool<Postgres>>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to read .env file");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // initialize tracing
    tracing_subscriber::fmt::init();

    // TODO postgresへの移行
    let pool = Arc::new(PgPool::connect(&database_url).await.unwrap());
    // build our application with a route
    // ApiImpl を Arc に包む（必要なら）
    let api_impl = ApiImpl { db_pool: pool }; // 実際の構造体を定義する
    let router = new(api_impl).layer(CorsLayer::new().allow_origin(Any));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {:?}", listener);
    axum::serve(listener, router).await.unwrap();
}

impl AsRef<ApiImpl> for ApiImpl {
    fn as_ref(&self) -> &ApiImpl {
        self
    }
}

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
