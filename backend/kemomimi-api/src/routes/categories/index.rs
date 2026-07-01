use crate::{db, libs::ApiImpl};
use async_trait::async_trait;
use axum::{extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use openapi::{
    apis::categories::{
        Categories, CategoriesCategoryIdDeleteResponse, CategoriesCategoryIdGetResponse,
        CategoriesCategoryIdPutResponse, CategoriesGetResponse, CategoriesPostResponse,
    },
    models,
};

#[async_trait]
impl Categories for ApiImpl {
    async fn categories_category_id_delete(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: models::CategoriesCategoryIdDeletePathParams,
    ) -> Result<CategoriesCategoryIdDeleteResponse, ()> {
        let deleted = db::categories::delete(&self.db_pool, &path_params.category_id)
            .await
            .map_err(|_| ())?;
        if deleted {
            Ok(CategoriesCategoryIdDeleteResponse::Status204)
        } else {
            Ok(CategoriesCategoryIdDeleteResponse::Status404)
        }
    }

    async fn categories_category_id_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: models::CategoriesCategoryIdGetPathParams,
    ) -> Result<CategoriesCategoryIdGetResponse, ()> {
        let row = db::categories::get(&self.db_pool, &path_params.category_id)
            .await
            .map_err(|_| ())?;
        match row {
            Some(row) => Ok(CategoriesCategoryIdGetResponse::Status200(db::categories::to_model(
                row,
            ))),
            None => Ok(CategoriesCategoryIdGetResponse::Status404),
        }
    }

    async fn categories_category_id_put(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: models::CategoriesCategoryIdPutPathParams,
        body: models::Category,
    ) -> Result<CategoriesCategoryIdPutResponse, ()> {
        if body.name.is_empty() {
            return Ok(CategoriesCategoryIdPutResponse::Status400);
        }

        let row = db::categories::update(
            &self.db_pool,
            &path_params.category_id,
            &body.name,
            body.remarks.as_deref(),
        )
        .await
        .map_err(|_| ())?;

        match row {
            Some(row) => Ok(CategoriesCategoryIdPutResponse::Status200(
                db::categories::to_model(row),
            )),
            None => Ok(CategoriesCategoryIdPutResponse::Status404),
        }
    }

    async fn categories_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<CategoriesGetResponse, ()> {
        let rows = db::categories::list(&self.db_pool).await.map_err(|_| ())?;
        Ok(CategoriesGetResponse::Status200(
            rows.into_iter().map(db::categories::to_model).collect(),
        ))
    }

    async fn categories_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        body: models::Category,
    ) -> Result<CategoriesPostResponse, ()> {
        if body.category_id.is_empty() || body.name.is_empty() {
            return Ok(CategoriesPostResponse::Status400);
        }

        let row = db::categories::insert(
            &self.db_pool,
            &body.category_id,
            &body.name,
            body.remarks.as_deref(),
        )
        .await
        .map_err(|_| ())?;

        Ok(CategoriesPostResponse::Status201(db::categories::to_model(row)))
    }
}
