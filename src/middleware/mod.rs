use crate::internal_vars::AUTH_HEADER;
use crate::services::wipfs_services::WipfsServices;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::ErrorUnauthorized;
use actix_web::middleware::Next;
use actix_web::web::Data;
use actix_web::Error;
use std::sync::Arc;

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let service = req.app_data::<Data<Arc<WipfsServices>>>().unwrap();
    if req.path().starts_with("/internal") {
        check_internal_auth(&req, &service)?;
    }

    println!("Accepted {} {:?}", req.path(), req.method());
    // Pre-processing

    // Call the next service in the middleware chain.
    let res = next.call(req).await?;

    // Post-processing
    Ok(res)
}

fn check_internal_auth(
    req: &ServiceRequest,
    service: &Data<Arc<WipfsServices>>,
) -> Result<(), Error> {
    let api_internal_key = service
        .secrets
        .get("API_INTERNAL_KEY")
        .ok_or_else(|| ErrorUnauthorized("Missing internal API key".to_string()))?;

    let token = req
        .headers()
        .get(AUTH_HEADER)
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| ErrorUnauthorized("API Key is incorrect".to_string()))?;

    if token != api_internal_key {
        return Err(ErrorUnauthorized("API Key is incorrect".to_string()));
    }

    Ok(())
}
