use crate::db::planetscale_driver::PlanetScaleDriver;
use crate::services::pin_service::PinServiceTrait;
use crate::services::storage_service::StorageService;
use crate::services::wvm_bundler_service::WvmBundlerService;
use std::sync::Arc;

pub struct WipfsServices {
    pub pin_service: Arc<dyn PinServiceTrait>,
    pub db_service: Arc<PlanetScaleDriver>,
    pub storage_service: Arc<StorageService>,
    pub wvm_bundler_service: Arc<WvmBundlerService>,
}

impl WipfsServices {
    pub fn new(
        pin_service: Arc<dyn PinServiceTrait>,
        db_service: Arc<PlanetScaleDriver>,
        storage_service: Arc<StorageService>,
        wvm_bundler_service: Arc<WvmBundlerService>,
    ) -> Self {
        Self {
            pin_service,
            db_service,
            storage_service,
            wvm_bundler_service,
        }
    }
}
