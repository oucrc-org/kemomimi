use crate::AppState;
use axum::{async_trait, extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use chrono::Utc;
use openapi::models::Product;
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

    /// 備品の購入コスト
    pub cost: Option<i32>,

    /// 製品ID
    pub product_id: Uuid,

    /// 購入日
    pub purchase_date: chrono::naive::NaiveDate,

    /// 承認日
    // pub approval_date: Option<chrono::naive::NaiveDate>,

    /// 耐用期限
    pub expiration_date: Option<chrono::naive::NaiveDate>,

    /// 現存しているか
    pub is_remaining: bool,

    // pub main_user_id: Option<String>,
    /// 備考欄
    pub remarks: Option<String>,
}

struct ProductCategoryRaw {
    /// 製品ID
    product_id: Uuid,

    /// カテゴリID
    category_id: String,

    /// カテゴリ名
    category_name: String,

    /// カテゴリの備考
    category_remarks: Option<String>,
}

#[async_trait]
impl PublicItems for AppState {
    #[doc = " 備品一覧取得."]
    #[doc = ""]
    #[doc = " PublicItemsGet - GET /public-items"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
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
                pi.product_id as "product_id!",
                pi.cost as "cost?",
                pi.purchase_date as "purchase_date!",
                pi.expiration_date as "expiration_date?",
                pi.is_remaining as "is_remaining!",
                pi.remarks as "remarks?"
            FROM
                public_item pi
            "#,
        )
        .fetch_all(&*self.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch public items: {:?}", e);
            ()
        })?;

        // `product_id` ごとにカテゴリを取得
        let product_ids: Vec<Uuid> = data.iter().map(|item| item.product_id).collect();

        let category_map = if !product_ids.is_empty() {
            let categories = query_as!(
                ProductCategoryRaw,
                r#"
                SELECT
                    pc.product_id as "product_id!",
                    c.category_id as "category_id!",
                    c.name as "category_name!",
                    c.remarks as "category_remarks?"
                FROM
                    product_category pc
                INNER JOIN
                    category c ON pc.category_id = c.category_id
                WHERE
                    pc.product_id = ANY($1)
                "#,
                &product_ids
            )
            .fetch_all(&*self.db_pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch categories: {:?}", e);
                ()
            })?;

            // `product_id` ごとにカテゴリをグループ化
            let mut map = std::collections::HashMap::new();
            for category in categories {
                map.entry(category.product_id)
                    .or_insert_with(Vec::new)
                    .push(models::Category {
                        category_id: category.category_id,
                        name: category.category_name,
                        remarks: category.category_remarks,
                    });
            }
            map
        } else {
            std::collections::HashMap::new()
        };

        // レスポンスデータを作成
        let data: Vec<models::PublicItem> = data
            .into_iter()
            .map(|item| models::PublicItem {
                public_item_id: item.public_item_id,
                name: item.name,
                category: category_map
                    .get(&item.product_id)
                    .and_then(|categories| categories.first().cloned()), // カテゴリを設定
                cost: item.cost,
                // approval_date: item.approval_date.map(|date| date.into()),
                approval_date: None,
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
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
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

        // product_id の存在チェック
        let product_exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM product WHERE product_id = $1
            ) AS "exists!"
            "#,
            body.product_id
        )
        .fetch_one(&mut tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to check product existence: {:?}", e);
            ()
        })?;

        if !product_exists {
            tracing::error!("Product not found: {:?}", body.product_id);
            return Ok(PublicItemsPostResponse::Status400);
        }

        // 備品情報の挿入
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
                expiration_date as "expiration_date?",
                is_remaining, 
                remarks
            "#,
            new_public_item_id,                     // 備品ID
            body.name,                              // 備品名
            body.product_id,                        // 製品ID
            body.cost,                              // 購入コスト
            body.purchase_date.map(|d| d.into()),   // 導入日
            body.expiration_date.map(|d| d.into()), // 耐用期限
            body.is_remaining.unwrap_or(true),      // 現存状態（NULLなら true とする）
            body.remarks                            // 備考
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

        // 製品に紐づくカテゴリ情報を取得
        let product_categories = {
            let category_rows = sqlx::query_as!(
                ProductCategoryRaw,
                r#"
                SELECT
                    pc.product_id as "product_id!",
                    c.category_id as "category_id!",
                    c.name as "category_name!",
                    c.remarks as "category_remarks?"
                FROM
                    product_category pc
                INNER JOIN
                    category c ON pc.category_id = c.category_id
                WHERE
                    pc.product_id = $1
                "#,
                body.product_id
            )
            .fetch_all(&*self.db_pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch categories for product: {:?}", e);
                ()
            })?;

            category_rows
                .into_iter()
                .map(|cat| models::Category {
                    category_id: cat.category_id,
                    name: cat.category_name,
                    remarks: cat.category_remarks,
                })
                .collect::<Vec<_>>()
        };

        // 製品情報を構築
        let product_instance = models::Product {
            product_id: product.product_id,
            name: product.name,
            model_number: product.model_number,
            product_url: product.product_url,
            categories: Some(product_categories), //
            main_users: Some(vec![]),             // Todo: 現時点では空
            remarks: product.remarks,
        };

        // レスポンス用備品詳細情報の作成
        let public_item_details = models::PublicItemDetails {
            public_item_id: inserted_item.public_item_id,
            name: inserted_item.name,
            product: product_instance,
            cost: inserted_item.cost,
            purchase_date: inserted_item.purchase_date,
            expiration_date: inserted_item.expiration_date,
            is_remaining: inserted_item.is_remaining,
            purchase_request_id: None, // 購入申請IDは未設定
            remarks: inserted_item.remarks,
        };

        Ok(PublicItemsPostResponse::Status201(public_item_details))
    }

    #[doc = " 備品削除."]
    #[doc = ""]
    #[doc = " PublicItemsPublicItemIdDelete - DELETE /public-items/{public-item-id}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
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
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
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
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
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
