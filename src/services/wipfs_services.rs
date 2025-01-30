use crate::services::pin_service::PinServiceTrait;
use std::sync::Arc;

pub struct WipfsServices {
    pub pin_service: Arc<dyn PinServiceTrait>,
}

impl WipfsServices {
    pub fn new(pin_service: Arc<dyn PinServiceTrait>) -> Self {
        Self { pin_service }
    }
}
