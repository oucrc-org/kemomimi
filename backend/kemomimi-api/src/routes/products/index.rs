use crate::{db, libs::ApiImpl};
use async_trait::async_trait;
use axum::{extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use openapi::{
    apis::products::{
        Products, ProductsGetResponse, ProductsPostResponse, ProductsProductIdDeleteResponse,
        ProductsProductIdGetResponse, ProductsProductIdPutResponse,
    },
    models,
};
use uuid::Uuid;

fn parse_product_id(product_id: &str) -> Uuid {
    Uuid::parse_str(product_id).unwrap_or_else(|_| Uuid::new_v4())
}

#[async_trait]
impl Products for ApiImpl {
    async fn products_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<ProductsGetResponse, ()> {
        let rows = db::products::list(&self.db_pool).await.map_err(|_| ())?;
        let mut products = Vec::with_capacity(rows.len());
        for row in rows {
            products.push(db::products::to_model(&self.db_pool, row).await.map_err(|_| ())?);
        }
        Ok(ProductsGetResponse::Status200(products))
    }

    async fn products_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        body: models::Product,
    ) -> Result<ProductsPostResponse, ()> {
        if body.name.is_empty() {
            return Ok(ProductsPostResponse::Status400);
        }

        let product_id = parse_product_id(&body.product_id);
        let row = db::products::insert(
            &self.db_pool,
            product_id,
            &body.name,
            body.model_number.as_deref(),
            body.product_url.as_deref(),
            body.remarks.as_deref(),
        )
        .await
        .map_err(|_| ())?;

        if let Some(categories) = &body.categiries {
            let category_ids: Vec<String> = categories.iter().map(|c| c.category_id.clone()).collect();
            db::products::set_categories(&self.db_pool, product_id, &category_ids)
                .await
                .map_err(|_| ())?;
        }

        if let Some(users) = &body.main_users {
            let user_ids: Vec<String> = users.iter().map(|u| u.user_id.clone()).collect();
            db::products::set_main_users(&self.db_pool, product_id, &user_ids)
                .await
                .map_err(|_| ())?;
        }

        let product = db::products::to_model(&self.db_pool, row).await.map_err(|_| ())?;
        Ok(ProductsPostResponse::Status201(product))
    }

    async fn products_product_id_delete(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: models::ProductsProductIdDeletePathParams,
    ) -> Result<ProductsProductIdDeleteResponse, ()> {
        let product_id = Uuid::parse_str(&path_params.product_id).map_err(|_| ())?;
        let deleted = db::products::delete(&self.db_pool, product_id)
            .await
            .map_err(|_| ())?;
        if deleted {
            Ok(ProductsProductIdDeleteResponse::Status204)
        } else {
            Ok(ProductsProductIdDeleteResponse::Status404)
        }
    }

    async fn products_product_id_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: models::ProductsProductIdGetPathParams,
    ) -> Result<ProductsProductIdGetResponse, ()> {
        let product_id = Uuid::parse_str(&path_params.product_id).map_err(|_| ())?;
        let row = db::products::get(&self.db_pool, product_id)
            .await
            .map_err(|_| ())?;
        match row {
            Some(row) => {
                let product = db::products::to_model(&self.db_pool, row).await.map_err(|_| ())?;
                Ok(ProductsProductIdGetResponse::Status200(product))
            }
            None => Ok(ProductsProductIdGetResponse::Status404),
        }
    }

    async fn products_product_id_put(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: models::ProductsProductIdPutPathParams,
        body: models::Product,
    ) -> Result<ProductsProductIdPutResponse, ()> {
        if body.name.is_empty() {
            return Ok(ProductsProductIdPutResponse::Status400);
        }

        let product_id = Uuid::parse_str(&path_params.product_id).map_err(|_| ())?;
        let row = db::products::update(
            &self.db_pool,
            product_id,
            &body.name,
            body.model_number.as_deref(),
            body.product_url.as_deref(),
            body.remarks.as_deref(),
        )
        .await
        .map_err(|_| ())?;

        let Some(row) = row else {
            return Ok(ProductsProductIdPutResponse::Status404);
        };

        if let Some(categories) = &body.categiries {
            let category_ids: Vec<String> = categories.iter().map(|c| c.category_id.clone()).collect();
            db::products::set_categories(&self.db_pool, product_id, &category_ids)
                .await
                .map_err(|_| ())?;
        }

        if let Some(users) = &body.main_users {
            let user_ids: Vec<String> = users.iter().map(|u| u.user_id.clone()).collect();
            db::products::set_main_users(&self.db_pool, product_id, &user_ids)
                .await
                .map_err(|_| ())?;
        }

        let product = db::products::to_model(&self.db_pool, row).await.map_err(|_| ())?;
        Ok(ProductsProductIdPutResponse::Status200(product))
    }
}
