mod actix_web_service;
mod handlers;
mod internal_vars;
mod middleware;
mod services;
mod structs;

use crate::actix_web_service::CustomShuttleActixWeb;
use crate::handlers::pin_handlers::configure_app;
use crate::services::pin_service::PinServiceTrait;
use crate::services::r#impl::wvm_pin::WvmPinService;
use crate::services::wipfs_services::WipfsServices;
use actix_web::web::Data;
use actix_web::{get, web::ServiceConfig};
use std::sync::Arc;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

fn get_services() -> Arc<WipfsServices> {
    let pin_service: Arc<dyn PinServiceTrait> = Arc::new(WvmPinService {});

    Arc::new(WipfsServices::new(pin_service))
}

#[shuttle_runtime::main]
async fn main() -> CustomShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(Data::new(get_services()));
        cfg.service(hello_world);
        configure_app(cfg);
    };

    Ok(config.into())
}
