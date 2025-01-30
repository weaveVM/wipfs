use crate::services::db_service::DbService;
use crate::services::pin_service::PinServiceTrait;
use std::sync::Arc;

pub struct WipfsServices {
    pub pin_service: Arc<dyn PinServiceTrait>,
    pub db_service: Arc<DbService>,
}

impl WipfsServices {
    pub fn new(pin_service: Arc<dyn PinServiceTrait>, db_service: Arc<DbService>) -> Self {
        Self {
            pin_service,
            db_service,
        }
    }
}
