use crate::db::schema::AccessKey;
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, HttpMessage, HttpRequest};

pub mod internal;
pub mod pin_handlers;

#[derive(Clone)]
pub struct CurrentUser(pub AccessKey);

fn extract_req_user(req: &HttpRequest) -> actix_web::Result<CurrentUser, Error> {
    let extensions = req.extensions();

    let auth = extensions
        .get::<CurrentUser>()
        .map(|e| e.clone())
        .ok_or_else(|| ErrorUnauthorized("Invalid Credentials"))?;
    Ok(auth)
}
