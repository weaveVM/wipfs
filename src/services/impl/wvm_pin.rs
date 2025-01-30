use crate::db::repo::pins::create_pin;
use crate::services::db_service::DbService;
use crate::services::pin_service::{GetPinsParams, PinServiceTrait};
use crate::structs::{Pin, PinResults, PinStatus};
use async_trait::async_trait;
use std::sync::Arc;

pub struct WvmPinService {
    pub(crate) db_service: Arc<DbService>,
}

#[async_trait]
impl PinServiceTrait for WvmPinService {
    async fn get_pins(&self, filters: &GetPinsParams) -> actix_web::Result<PinResults> {
        println!("get_pins");
        todo!()
    }

    async fn add_pin(&self, pin: Pin) -> actix_web::Result<PinStatus> {
        let mut conn = self.db_service.db_pool.get().unwrap();
        create_pin(&mut conn, &pin.cid, 0);
        todo!()
    }

    async fn get_pin_by_request_id(&self, request_id: &str) -> actix_web::Result<PinStatus> {
        println!("get_pin_by_request_id");
        todo!()
    }

    async fn replace_pin(&self, request_id: &str, pin: Pin) -> actix_web::Result<PinStatus> {
        println!("replace_pin");
        todo!()
    }

    async fn delete_pin(&self, request_id: &str) -> actix_web::Result<()> {
        println!("delete_pin");
        todo!()
    }
}
