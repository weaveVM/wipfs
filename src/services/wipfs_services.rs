use crate::services::db_service::DbService;
use crate::services::pin_service::PinServiceTrait;
use crate::services::storage_service::StorageService;
use std::sync::Arc;

pub struct WipfsServices {
    pub pin_service: Arc<dyn PinServiceTrait>,
    pub db_service: Arc<DbService>,
    pub storage_service: Arc<StorageService>,
}

impl WipfsServices {
    pub fn new(
        pin_service: Arc<dyn PinServiceTrait>,
        db_service: Arc<DbService>,
        storage_service: Arc<StorageService>,
    ) -> Self {
        Self {
            pin_service,
            db_service,
            storage_service,
        }
    }
}
