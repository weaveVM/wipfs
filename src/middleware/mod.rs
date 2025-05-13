use crate::handlers::CurrentUser;
use crate::internal_vars::AUTH_HEADER;
use crate::services::wipfs_services::WipfsServices;
use crate::utils::http::get_internal_call;
use crate::internal_vars::AUTH_HOST;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::ErrorUnauthorized;
use actix_web::middleware::Next;
use actix_web::web::Data;
use actix_web::{Error, HttpMessage, HttpResponse};
use std::sync::Arc;

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let service = req.app_data::<Data<Arc<WipfsServices>>>().unwrap();
    if req.path().starts_with("/internal") {
        check_internal_auth(&req, &service)?;
        let url = format!("{}{}", *AUTH_HOST, req.path()); 
        let response = get_internal_call::<serde_json::Value>(url).unwrap(); 
        let http_resp = HttpResponse::Ok().json(response);
        return Ok(ServiceResponse::new(
            req.into_parts().0,
            http_resp.map_into_boxed_body()
        ));

    } else if req.path().starts_with("/pins") {
        let is_valid_key = check_user_auth(&req, &service).await.unwrap_or(false);
        if !is_valid_key {
            return Err(ErrorUnauthorized("API key is invalid".to_string()));
        }
    }

    println!("Accepted {} {:?} {}", req.path(), req.method(), req.connection_info().host());
    let res = next.call(req).await?;
    return Ok(res.map_into_boxed_body());
}

async fn check_user_auth(
    req: &ServiceRequest,
    service: &Data<Arc<WipfsServices>>,
) -> Result<bool, Error> {
    let token = req
        .headers()
        .get("authorization")
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| ErrorUnauthorized("API Key is incorrect".to_string()))?;

    // We don't need bearer tokens here.
    let token = token.replace("Bearer ", "");

    let access_key = service.auth_service.verify_access(token).await;

    if let Some(access_key) = access_key {
        req.extensions_mut().insert(CurrentUser(access_key));
        Ok(true)
    } else {
        Ok(false)
    }
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
