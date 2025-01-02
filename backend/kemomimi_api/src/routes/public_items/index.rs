use crate::libs::ApiImpl;
use axum::{async_trait, extract::Host, http::Method};
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
use sqlx::{query_as, types::Uuid};
use tracing::info;

#[derive(Debug, Clone, PartialEq)]
struct PublicItemRaw {
    /// 備品のユニークID
    pub public_item_id: Uuid,

    /// 備品名
    pub name: String,
    pub category_id: Option<String>,

    /// 備品の購入コスト
    pub cost: Option<i32>,

    /// 承認日
    pub approval_date: Option<sqlx::types::time::Date>,

    /// 耐用期限
    pub expiration_date: Option<sqlx::types::time::Date>,

    /// 現存しているか
    pub is_remaining: bool,

    // pub main_user_id: Option<String>,
    /// 備考欄
    pub remarks: Option<String>,
}

#[async_trait]
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
    #[tracing::instrument]
    async fn public_items_get<'life0>(
        &'life0 self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        query_params: models::PublicItemsGetQueryParams,
    ) -> Result<PublicItemsGetResponse, ()>
    where
        'life0: 'async_trait,
    {
        let data = query_as!(
            PublicItemRaw,
            r#"
            SELECT
                pi.public_item_id as "public_item_id!",
                pi.name as "name!",
                c.category_id as "category_id?",
                pi.cost as "cost?",
                pi.purchase_date as "approval_date?",
                pi.expiration_date as "expiration_date?",
                pi.is_remaining as "is_remaining!",
                pi.remarks as "remarks?"
            FROM
                public_item pi
            LEFT JOIN
                product p ON pi.product_id = p.product_id
            LEFT JOIN
                 product_category pc ON p.product_id = pc.product_id
            LEFT JOIN
                category c ON pc.category_id = c.category_id
            "#
        )
        .fetch_all(&*self.db_pool)
        .await
        .unwrap();

        let data: Vec<PublicItem> = data
            .into_iter()
            .map(|item| PublicItem {
                public_item_id: item.public_item_id.to_string(),
                name: item.name,
                category: None, // TODO
                cost: item.cost,
                // approval_date: item.approval_date.map(|date| date.into()),
                approval_date: None,
                // expiration_date: item.expiration_date.map(|date| date.into()),
                expiration_date: None,
                is_remaining: item.is_remaining,
                main_user: None, // ユーザー情報が不明なため、一旦 None で固定
                remarks: item.remarks,
            })
            .collect();
        Ok(PublicItemsGetResponse::Status200(data))
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
    async fn public_items_post<'life0>(
        &'life0 self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        body: models::PublicItemEntry,
    ) -> Result<PublicItemsPostResponse, ()>
    where
        'life0: 'async_trait,
    {
        let public_item_id = Uuid::now_v7();

        let public_item_id = query_as!(
            Uuid,
            r#"
            INSERT INTO public_item (
                public_item_id,
                name,
                cost,
                purchase_date,
                expiration_date,
                is_remaining,
                remarks
            )
            VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6,
                $7
            )
            RETURNING public_item_id;
            "#
        );
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
    async fn public_items_public_item_id_delete<'life0>(
        &'life0 self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::PublicItemsPublicItemIdDeletePathParams,
    ) -> Result<PublicItemsPublicItemIdDeleteResponse, ()>
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
    async fn public_items_public_item_id_get<'life0>(
        &'life0 self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::PublicItemsPublicItemIdGetPathParams,
    ) -> Result<PublicItemsPublicItemIdGetResponse, ()>
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
    async fn public_items_public_item_id_put<'life0>(
        &'life0 self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::PublicItemsPublicItemIdPutPathParams,
        body: models::PublicItemDetails,
    ) -> Result<PublicItemsPublicItemIdPutResponse, ()>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }
}
