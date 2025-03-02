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
use openapi::models::Product;
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
                public_item_id: item.public_item_id,
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
        let new_public_item_id = Uuid::now_v7(); // 新規の備品IDを生成

        // トランザクション開始
        let mut tx = self.db_pool.begin().await.map_err(|e| {
            tracing::error!("Failed to start transaction: {:?}", e);
            ()
        })?;

        // 備品情報を挿入
        let inserted_item = sqlx::query_as!(
            PublicItemRaw,
            r#"
            INSERT INTO public_item (
                public_item_id,
                name,
                product_id,
                cost,
                purchase_date,
                expiration_date,
                is_remaining,
                remarks
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING 
                public_item_id, 
                name, 
                product_id, 
                cost, 
                purchase_date, 
                expiration_date, 
                is_remaining, 
                remarks
            "#,
            new_public_item_id,   // $1: 備品ID
            body.name,            // $2: 備品名
            body.product_id,      // $3: 製品ID（既存の製品を参照）
            body.cost,            // $4: 購入コスト
            body.purchase_date,   // $5: 導入日
            body.expiration_date, // $6: 耐用期限
            body.is_remaining,    // $7: 現存状態
            body.remarks          // $8: 備考
        )
        .fetch_one(&mut tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to insert public item: {:?}", e);
            ()
        })?;

        // 製品情報を取得
        let product = sqlx::query_as!(
            Product,
            r#"
            SELECT 
                product_id,
                name,
                model_number,
                product_url,
                remarks
            FROM product
            WHERE product_id = $1
            "#,
            body.product_id
        )
        .fetch_optional(&mut tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch product: {:?}", e);
            ()
        })?
        .ok_or_else(|| {
            tracing::error!("Product not found: {:?}", body.product_id);
            ()
        })?;

        // トランザクションのコミット
        tx.commit().await.map_err(|e| {
            tracing::error!("Failed to commit transaction: {:?}", e);
            ()
        })?;

        // レスポンスデータを作成
        let public_item_details = models::PublicItemDetails {
            public_item_id: inserted_item.public_item_id.to_string(),
            name: inserted_item.name,
            product: Some(models::Product {
                product_id: product.product_id.to_string(),
                name: product.name,
                model_number: product.model_number,
                product_url: product.product_url,
                categiries: Some(vec![]), // カテゴリ情報を空リストで設定
                main_users: Some(vec![]), // main_users も空で設定
                remarks: product.remarks,
            }),
            cost: inserted_item.cost,
            purchase_date: inserted_item.purchase_date,
            expiration_date: inserted_item.expiration_date,
            is_remaining: inserted_item.is_remaining,
            purchase_request_id: None, // 購入申請IDは未設定
            remarks: inserted_item.remarks,
        };

        // 成功レスポンスを返却
        Ok(PublicItemsPostResponse::Status201(public_item_details))
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
