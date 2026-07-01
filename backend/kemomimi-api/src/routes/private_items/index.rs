use crate::{db, libs::ApiImpl};
use async_trait::async_trait;
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

#[async_trait]
impl PrivateItems for ApiImpl {
    async fn private_items_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<PrivateItemsGetResponse, ()> {
        let rows = db::private_items::list(&self.db_pool).await.map_err(|_| ())?;
        Ok(PrivateItemsGetResponse::Status200(
            rows.into_iter().map(db::private_items::to_model).collect(),
        ))
    }

    async fn private_items_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        body: models::PrivateItem,
    ) -> Result<PrivateItemsPostResponse, ()> {
        if body.private_item_id.is_empty() {
            return Ok(PrivateItemsPostResponse::Status400);
        }

        let row = db::private_items::insert(
            &self.db_pool,
            &body.private_item_id,
            body.name.as_deref().unwrap_or(""),
            body.owner_id.as_deref(),
            body.post_grad_treat_id.as_deref(),
            body.model_number.as_deref(),
            body.is_remaining.unwrap_or(true),
            body.remarks.as_deref(),
        )
        .await
        .map_err(|_| ())?;

        Ok(PrivateItemsPostResponse::Status201(db::private_items::to_model(row)))
    }

    async fn private_items_private_item_id_delete(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: models::PrivateItemsPrivateItemIdDeletePathParams,
    ) -> Result<PrivateItemsPrivateItemIdDeleteResponse, ()> {
        let deleted =
            db::private_items::delete(&self.db_pool, &path_params.private_item_id)
                .await
                .map_err(|_| ())?;
        if deleted {
            Ok(PrivateItemsPrivateItemIdDeleteResponse::Status204)
        } else {
            Ok(PrivateItemsPrivateItemIdDeleteResponse::Status404)
        }
    }

    async fn private_items_private_item_id_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: models::PrivateItemsPrivateItemIdGetPathParams,
    ) -> Result<PrivateItemsPrivateItemIdGetResponse, ()> {
        let row = db::private_items::get(&self.db_pool, &path_params.private_item_id)
            .await
            .map_err(|_| ())?;
        match row {
            Some(row) => Ok(PrivateItemsPrivateItemIdGetResponse::Status200(
                db::private_items::to_model(row),
            )),
            None => Ok(PrivateItemsPrivateItemIdGetResponse::Status404),
        }
    }

    async fn private_items_private_item_id_put(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: models::PrivateItemsPrivateItemIdPutPathParams,
        body: models::PrivateItem,
    ) -> Result<PrivateItemsPrivateItemIdPutResponse, ()> {
        let row = db::private_items::update(
            &self.db_pool,
            &path_params.private_item_id,
            body.name.as_deref().unwrap_or(""),
            body.owner_id.as_deref(),
            body.post_grad_treat_id.as_deref(),
            body.model_number.as_deref(),
            body.is_remaining.unwrap_or(true),
            body.remarks.as_deref(),
        )
        .await
        .map_err(|_| ())?;

        match row {
            Some(row) => Ok(PrivateItemsPrivateItemIdPutResponse::Status200(
                db::private_items::to_model(row),
            )),
            None => Ok(PrivateItemsPrivateItemIdPutResponse::Status404),
        }
    }
}
