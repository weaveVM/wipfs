use crate::handlers::{extract_req_user, CurrentUser};
use crate::services::pin_service::{GetPinsParams, PinServiceTrait};
use crate::services::wipfs_services::WipfsServices;
use crate::structs::{CreatePin, Pin, PinResults, PinStatus};
use crate::utils::parse_query_string;
use actix_web::error::ErrorUnauthorized;
use actix_web::web::{Path, ServiceConfig};
use actix_web::{
    delete, get, post,
    web::{Data, Json, Query},
    Error, HttpMessage, HttpRequest, HttpResponse, Result,
};
use std::sync::Arc;
// Handler functions

#[get("/pins")]
pub async fn get_pins(
    service: Data<Arc<WipfsServices>>,
    req: HttpRequest,
) -> Result<Json<PinResults>> {
    let auth = extract_req_user(&req)?;

    let mut params = parse_query_string(req.query_string());

    // params
    params.created_by = Some(auth.0.owner_id);

    let result = service.pin_service.get_pins(&params).await?;
    Ok(Json(result))
}

#[post("/pins")]
pub async fn add_pin(
    req: HttpRequest,
    service: Data<Arc<WipfsServices>>,
    pin: Json<Pin>,
) -> Result<HttpResponse> {
    let pin = pin.into_inner();

    let auth = extract_req_user(&req)?;

    let create_pin = CreatePin {
        pin,
        created_by: auth.0.owner_id,
        user_token: auth.0.access_key.clone(),
    };

    let result = service.pin_service.add_pin(create_pin).await?;
    Ok(HttpResponse::Accepted().json(result))
}

#[get("/pins/{request_id}")]
pub async fn get_pin_by_id(
    service: Data<Arc<WipfsServices>>,
    request_id: Path<String>,
) -> Result<Json<PinStatus>> {
    let result = service
        .pin_service
        .get_pin_by_request_id(&request_id.into_inner())
        .await?;
    Ok(Json(result))
}

#[post("/pins/{request_id}")]
pub async fn replace_pin(
    service: Data<Arc<WipfsServices>>,
    request_id: Path<String>,
    pin: Json<Pin>,
) -> Result<HttpResponse> {
    let result = service
        .pin_service
        .replace_pin(&request_id.into_inner(), pin.into_inner())
        .await?;
    Ok(HttpResponse::Accepted().json(result))
}

#[delete("/pins/{request_id}")]
pub async fn delete_pin(
    service: Data<Arc<WipfsServices>>,
    request_id: Path<String>,
) -> Result<HttpResponse> {
    service
        .pin_service
        .delete_pin(&request_id.into_inner())
        .await?;
    Ok(HttpResponse::Accepted().finish())
}

// App configuration function
pub fn configure_app(cfg: &mut ServiceConfig) {
    cfg.service(get_pins)
        .service(add_pin)
        .service(get_pin_by_id)
        .service(replace_pin)
        .service(delete_pin);
}
