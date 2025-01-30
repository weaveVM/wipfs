use crate::db::repo::pins::create_pin;
use crate::internal_vars::IPFS_HOST;
use crate::services::db_service::DbService;
use crate::services::pin_service::{GetPinsParams, PinServiceTrait};
use crate::services::storage_service::StorageService;
use crate::structs::{Pin, PinMeta, PinResults, PinStatus};
use async_trait::async_trait;
use std::io::Read;
use std::sync::Arc;

pub struct WvmPinService {
    pub(crate) db_service: Arc<DbService>,
    pub(crate) storage_service: Arc<StorageService>,
}

impl WvmPinService {
    fn fetch_ipfs_file(&self, cid: &str) -> Option<Vec<u8>> {
        let url = format!("{}/ipfs/{}", &*IPFS_HOST, cid);

        // Perform HTTP GET request
        let response = ureq::get(&url).call().ok()?; // Convert Result to Option
        let mut bytes = Vec::new();

        // Read response body into Vec<u8>
        let body = response.into_body();
        let mut reader = body.into_reader();
        reader.read_to_end(&mut bytes).ok()?;

        Some(bytes)
    }
}

#[async_trait]
impl PinServiceTrait for WvmPinService {
    async fn get_pins(&self, filters: &GetPinsParams) -> actix_web::Result<PinResults> {
        println!("get_pins");
        todo!()
    }

    async fn add_pin(&self, pin: Pin) -> actix_web::Result<PinStatus> {
        let mut conn = self.db_service.db_pool.get().unwrap();
        let insert_pin_data = create_pin(&mut conn, &pin.cid, 0);

        if let Ok(_) = insert_pin_data {
            let metadata = pin.meta.unwrap_or_else(|| PinMeta::default());

            let content_type = metadata
                .0
                .get("content-type")
                .map(|e| e.clone())
                .unwrap_or("application/octet-stream".to_string());

            let file = self.fetch_ipfs_file(&pin.cid);
            if let Some(bytes) = file {
                let upload_to_bucket = self
                    .storage_service
                    .upload(bytes, &pin.cid, &content_type)
                    .await;
                if upload_to_bucket.is_ok() {
                    // Send to bundler
                }
            }
        }

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
