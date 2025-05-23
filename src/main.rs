mod actix_web_service;
mod db;
mod handlers;
mod internal_vars;
mod middleware;
mod services;
mod structs;
mod utils;

use crate::actix_web_service::CustomShuttleActixWeb;
use crate::db::planetscale_driver::PlanetScaleDriver;
use crate::handlers::pin_handlers::configure_app;
use crate::services::auth_service::AuthService;
use crate::services::pin_service::PinServiceTrait;
use crate::services::r#impl::wvm_pin::WvmPinService;
use crate::services::storage_service;
use crate::services::storage_service::StorageService;
use crate::services::wipfs_services::WipfsServices;
use crate::services::wvm_bundler_service::WvmBundlerService;
use actix_web::http::header;
use actix_web::web::Data;
use actix_web::{get, web::ServiceConfig};
use std::sync::Arc;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

async fn get_services(secrets: shuttle_runtime::SecretStore) -> Arc<WipfsServices> {
    let db_driver = Arc::new(PlanetScaleDriver::from(&secrets));

    let storage_service = {
        let bucket_name = secrets.get("BUCKET_NAME").unwrap();
        let credentials = secrets.get("SERVICE_ACCOUNT_JSON").unwrap();
        std::env::set_var("SERVICE_ACCOUNT_JSON", credentials.trim());

        Arc::new(StorageService::new(bucket_name).await)
    };

    let bundler_service = {
        let priv_key = secrets.get("BUNDLER_PRIV_KEY").unwrap();
        Arc::new(WvmBundlerService::new(priv_key))
    };

    let pin_service: Arc<dyn PinServiceTrait> = Arc::new(WvmPinService {
        db_service: db_driver.clone(),
        storage_service: storage_service.clone(),
        wvm_bundler_service: bundler_service.clone(),
    });

    let auth_service = Arc::new(AuthService {
        db_service: db_driver.clone(),
    });

    Arc::new(WipfsServices::new(
        pin_service,
        db_driver,
        storage_service,
        bundler_service,
        auth_service,
        secrets,
    ))
}

fn configure_env_vars(secrets: &shuttle_runtime::SecretStore) {
    unsafe {
        std::env::set_var(
            "API_INTERNAL_KEY",
            secrets.get("API_INTERNAL_KEY").unwrap_or("".to_string()),
        );
    }
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> CustomShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    configure_env_vars(&secrets);

    let service_box = get_services(secrets.clone()).await;
    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(actix_web::web::Data::new(
            actix_web::middleware::DefaultHeaders::new()
                .add((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
                .add((
                    header::ACCESS_CONTROL_ALLOW_METHODS,
                    "GET, POST, PUT, DELETE, OPTIONS",
                ))
                .add((
                    header::ACCESS_CONTROL_ALLOW_HEADERS,
                    "Content-Type, Authorization",
                )),
        ));

        cfg.app_data(Data::new(service_box));
        cfg.service(hello_world);
        configure_app(cfg);
    };

    Ok(config.into())
}
