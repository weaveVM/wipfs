use crate::services::pin_service::{GetPinsParams, PinServiceTrait};
use crate::services::wipfs_services::WipfsServices;
use crate::structs::{Pin, PinResults, PinStatus};
use actix_web::web::{Path, ServiceConfig};
use actix_web::{
    delete, get, post,
    web::{Data, Json, Query},
    HttpResponse, Result,
};
use std::sync::Arc;
// Handler functions

#[get("/pins")]
pub async fn get_pins(
    service: Data<Arc<WipfsServices>>,
    params: Query<GetPinsParams>,
) -> Result<Json<PinResults>> {
    println!("Pins called");
    let result = service.pin_service.get_pins(&params.into_inner()).await?;
    Ok(Json(result))
}

#[post("/pins")]
pub async fn add_pin(service: Data<Arc<WipfsServices>>, pin: Json<Pin>) -> Result<HttpResponse> {
    let pin = pin.into_inner();
    println!("{:?}", pin);
    let result = service.pin_service.add_pin(pin).await?;
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
