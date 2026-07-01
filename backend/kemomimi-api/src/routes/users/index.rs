use crate::{db, libs::ApiImpl};
use async_trait::async_trait;
use axum::{extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use openapi::{
    apis::users::{
        Users, UsersGetResponse, UsersPostResponse, UsersUserIdDeleteResponse,
        UsersUserIdGetResponse, UsersUserIdPutResponse,
    },
    models,
};

#[async_trait]
impl Users for ApiImpl {
    async fn users_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<UsersGetResponse, ()> {
        let rows = db::users::list(&self.db_pool).await.map_err(|_| ())?;
        Ok(UsersGetResponse::Status200(
            rows.into_iter().map(db::users::to_model).collect(),
        ))
    }

    async fn users_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        body: models::User,
    ) -> Result<UsersPostResponse, ()> {
        if body.user_id.is_empty() || body.handle_name.is_empty() || body.screen_name.is_empty() {
            return Ok(UsersPostResponse::Status400);
        }

        let row = db::users::insert(
            &self.db_pool,
            &body.user_id,
            &body.handle_name,
            &body.screen_name,
            body.slack_id.as_deref(),
            body.is_admin.unwrap_or(false),
            body.is_member.unwrap_or(true),
            body.graduation_date,
            body.remarks.as_deref(),
        )
        .await
        .map_err(|_| ())?;

        Ok(UsersPostResponse::Status201(db::users::to_model(row)))
    }

    async fn users_user_id_delete(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: models::UsersUserIdDeletePathParams,
    ) -> Result<UsersUserIdDeleteResponse, ()> {
        let deleted = db::users::delete(&self.db_pool, &path_params.user_id)
            .await
            .map_err(|_| ())?;
        if deleted {
            Ok(UsersUserIdDeleteResponse::Status204)
        } else {
            Ok(UsersUserIdDeleteResponse::Status404)
        }
    }

    async fn users_user_id_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: models::UsersUserIdGetPathParams,
    ) -> Result<UsersUserIdGetResponse, ()> {
        let row = db::users::get(&self.db_pool, &path_params.user_id)
            .await
            .map_err(|_| ())?;
        match row {
            Some(row) => Ok(UsersUserIdGetResponse::Status200(db::users::to_model(row))),
            None => Ok(UsersUserIdGetResponse::Status404),
        }
    }

    async fn users_user_id_put(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: models::UsersUserIdPutPathParams,
        body: models::User,
    ) -> Result<UsersUserIdPutResponse, ()> {
        if body.handle_name.is_empty() || body.screen_name.is_empty() {
            return Ok(UsersUserIdPutResponse::Status400);
        }

        let row = db::users::update(
            &self.db_pool,
            &path_params.user_id,
            &body.handle_name,
            &body.screen_name,
            body.slack_id.as_deref(),
            body.is_admin.unwrap_or(false),
            body.is_member.unwrap_or(true),
            body.graduation_date,
            body.remarks.as_deref(),
        )
        .await
        .map_err(|_| ())?;

        match row {
            Some(row) => Ok(UsersUserIdPutResponse::Status200(db::users::to_model(row))),
            None => Ok(UsersUserIdPutResponse::Status404),
        }
    }
}
