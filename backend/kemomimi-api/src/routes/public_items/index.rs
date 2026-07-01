use crate::{db, libs::ApiImpl};
use async_trait::async_trait;
use axum::{extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use chrono::Utc;
use openapi::{
    apis::public_items::{
        PublicItems, PublicItemsGetResponse, PublicItemsPostResponse,
        PublicItemsPublicItemIdDeleteResponse, PublicItemsPublicItemIdGetResponse,
        PublicItemsPublicItemIdPutResponse,
    },
    models,
};
use uuid::Uuid;

#[async_trait]
impl PublicItems for ApiImpl {
    async fn public_items_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        query_params: models::PublicItemsGetQueryParams,
    ) -> Result<PublicItemsGetResponse, ()> {
        let rows = db::public_items::list(
            &self.db_pool,
            db::public_items::ListParams {
                search: query_params.search.as_deref(),
                sort: query_params.sort.as_deref(),
                filter: query_params.filter.as_deref(),
                filter_value: None,
            },
        )
        .await
        .map_err(|_| ())?;

        Ok(PublicItemsGetResponse::Status200(
            rows.iter().map(db::public_items::to_public_item).collect(),
        ))
    }

    async fn public_items_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        body: models::PublicItemEntry,
    ) -> Result<PublicItemsPostResponse, ()> {
        if body.name.is_empty() {
            return Ok(PublicItemsPostResponse::Status400);
        }

        let product_id = db::products::find_or_create_by_name(&self.db_pool, &body.name)
            .await
            .map_err(|_| ())?;

        let public_item_id = Uuid::new_v4();
        let purchase_date = body.purchase_date.unwrap_or_else(|| Utc::now().date_naive());
        let is_remaining = body.is_remaining.unwrap_or(true);

        let row = db::public_items::insert(
            &self.db_pool,
            public_item_id,
            product_id,
            &body.name,
            body.cost,
            purchase_date,
            body.expiration_date,
            is_remaining,
            body.remarks.as_deref(),
        )
        .await
        .map_err(|_| ())?;

        let details = db::public_items::to_details(
            &self.db_pool,
            row,
            Some(body.purchase_request_id),
        )
        .await
        .map_err(|_| ())?;

        Ok(PublicItemsPostResponse::Status201(details))
    }

    async fn public_items_public_item_id_delete(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: models::PublicItemsPublicItemIdDeletePathParams,
    ) -> Result<PublicItemsPublicItemIdDeleteResponse, ()> {
        let public_item_id =
            Uuid::parse_str(&path_params.public_item_id).map_err(|_| ())?;
        let deleted = db::public_items::delete(&self.db_pool, public_item_id)
            .await
            .map_err(|_| ())?;
        if deleted {
            Ok(PublicItemsPublicItemIdDeleteResponse::Status204)
        } else {
            Ok(PublicItemsPublicItemIdDeleteResponse::Status404)
        }
    }

    async fn public_items_public_item_id_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: models::PublicItemsPublicItemIdGetPathParams,
    ) -> Result<PublicItemsPublicItemIdGetResponse, ()> {
        let public_item_id =
            Uuid::parse_str(&path_params.public_item_id).map_err(|_| ())?;
        let row = db::public_items::get(&self.db_pool, public_item_id)
            .await
            .map_err(|_| ())?;
        match row {
            Some(row) => {
                let details = db::public_items::to_details(&self.db_pool, row, None)
                    .await
                    .map_err(|_| ())?;
                Ok(PublicItemsPublicItemIdGetResponse::Status200(details))
            }
            None => Ok(PublicItemsPublicItemIdGetResponse::Status404),
        }
    }

    async fn public_items_public_item_id_put(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: models::PublicItemsPublicItemIdPutPathParams,
        body: models::PublicItemDetails,
    ) -> Result<PublicItemsPublicItemIdPutResponse, ()> {
        let public_item_id =
            Uuid::parse_str(&path_params.public_item_id).map_err(|_| ())?;

        let row = db::public_items::update(
            &self.db_pool,
            public_item_id,
            body.name.as_deref(),
            body.cost,
            body.purchase_date,
            body.expiration_date,
            body.is_remaining,
            body.remarks.as_deref(),
        )
        .await
        .map_err(|_| ())?;

        match row {
            Some(row) => {
                let details = db::public_items::to_details(
                    &self.db_pool,
                    row,
                    body.purchase_request_id,
                )
                .await
                .map_err(|_| ())?;
                Ok(PublicItemsPublicItemIdPutResponse::Status200(details))
            }
            None => Ok(PublicItemsPublicItemIdPutResponse::Status404),
        }
    }
}
