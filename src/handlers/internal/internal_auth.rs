use crate::db::repo::auth::{NewAccessKeys, NewAccount};
use crate::services::wipfs_services::WipfsServices;
use actix_web::error::ErrorUnauthorized;
use actix_web::web::{Data, Json, ServiceConfig};
use actix_web::{post, HttpResponse};
use std::sync::Arc;

#[post("/internal/account")]
pub async fn create_account(
    service: Data<Arc<WipfsServices>>,
    account: Json<NewAccount>,
) -> actix_web::Result<HttpResponse> {
    let add_acct = service
        .auth_service
        .create_account(account.into_inner())
        .await
        .map_err(|e| ErrorUnauthorized(e))?;

    Ok(HttpResponse::Accepted().finish())
}

#[post("/internal/access-key")]
pub async fn create_access_key(
    service: Data<Arc<WipfsServices>>,
    access_key: Json<NewAccessKeys>,
) -> actix_web::Result<HttpResponse> {
    let add_access_key = service
        .auth_service
        .create_access_key(access_key.into_inner())
        .await
        .map_err(|e| ErrorUnauthorized(e))?;

    Ok(HttpResponse::Accepted().finish())
}

pub fn configure_internal_endpoints(cfg: &mut ServiceConfig) {
    cfg.service(create_account).service(create_access_key);
}
