use crate::db::repo::auth::{NewAccessKeys, NewAccount};
use crate::services::wipfs_services::WipfsServices;
use actix_web::error::{ErrorNotFound, ErrorUnauthorized};
use actix_web::web::{Data, Json, Path, ServiceConfig};
use actix_web::{get, post, HttpResponse};
use std::sync::Arc;

#[get("/internal/account/{account_name}")]
pub async fn get_account(
    service: Data<Arc<WipfsServices>>,
    account_name: Path<String>,
) -> actix_web::Result<HttpResponse> {
    let add_acct = service
        .auth_service
        .find_account(account_name.into_inner())
        .await
        .map_err(|e| ErrorNotFound(e))?;

    Ok(HttpResponse::Ok().json(add_acct))
}

#[get("/internal/keys/{owner_id}")]
pub async fn get_keys(
    service: Data<Arc<WipfsServices>>,
    owner_id: Path<i64>,
) -> actix_web::Result<HttpResponse> {
    let add_acct = service
        .auth_service
        .list_keys_for_owner(owner_id.into_inner())
        .await
        .map_err(|e| ErrorNotFound(e))?;

    Ok(HttpResponse::Ok().json(add_acct))
}

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
    cfg.service(get_keys)
        .service(get_account)
        .service(create_account)
        .service(create_access_key);
}
