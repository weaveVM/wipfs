use crate::db::planetscale_driver::PlanetScaleDriver;
use crate::db::repo::pins::{create_pin, find_pin, find_pins};
use crate::db::DATE_FORMAT_MYSQL;
use crate::internal_vars::IPFS_HOST;
use crate::services::pin_service::{GetPinsParams, PinServiceTrait};
use crate::services::storage_service::StorageService;
use crate::services::wvm_bundler_service::WvmBundlerService;
use crate::structs::{CreatePin, Pin, PinMeta, PinResults, PinStatus, Status, StatusInfo};
use actix_web::error::{ErrorInternalServerError, ErrorNotFound};
use async_trait::async_trait;
use bundler::utils::core::bundle::Bundle;
use chrono::{DateTime, Local, NaiveDateTime, Utc};
use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;
use uuid::Uuid;

pub struct WvmPinService {
    pub(crate) db_service: Arc<PlanetScaleDriver>,
    pub(crate) storage_service: Arc<StorageService>,
    pub(crate) wvm_bundler_service: Arc<WvmBundlerService>,
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

    fn get_new_status_info(&self, bundlr_tx: String) -> StatusInfo {
        let mut map = HashMap::new();
        map.insert("Arweave-Tx".to_string(), bundlr_tx);
        StatusInfo(map)
    }
}

#[async_trait]
impl PinServiceTrait for WvmPinService {
    async fn get_pins(&self, filters: &GetPinsParams) -> actix_web::Result<PinResults> {
        let conn = self.db_service.get_conn();
        let pins = find_pins(conn, &filters)
            .await
            .map_err(|e| ErrorInternalServerError(e))?;
        println!("{:?}", pins);
        Ok(PinResults {
            count: pins.len() as i32,
            results: pins
                .into_iter()
                .map(|db_file| PinStatus {
                    request_id: db_file.req_id,
                    status: Status::Pinned,
                    created: DateTime::parse_from_str(&db_file.created_at, DATE_FORMAT_MYSQL)
                        .unwrap()
                        .with_timezone(&Utc),
                    pin: Pin {
                        cid: db_file.cid,
                        name: Some(db_file.name),
                        origins: None,
                        meta: None,
                    },
                    delegates: vec![],
                    info: None,
                })
                .collect(),
        })
    }

    async fn add_pin(&self, data: CreatePin) -> actix_web::Result<PinStatus> {
        let CreatePin { created_by, pin } = data;

        let conn = self.db_service.get_conn();
        let req_id = Uuid::new_v4().to_string();

        if let Some(bytes) = self.fetch_ipfs_file(&pin.cid) {
            println!("Bytes found");
            let metadata = pin.meta.clone().unwrap_or_else(|| PinMeta::default());
            let content_type = metadata
                .0
                .get("content-type")
                .map(|e| e.clone())
                .unwrap_or("application/octet-stream".to_string());

            let upload_to_bucket = self
                .storage_service
                .upload(bytes.clone(), &pin.cid, &content_type)
                .await;

            if upload_to_bucket.is_ok() {
                let len = bytes.len();
                let send = self
                    .wvm_bundler_service
                    .send(pin.cid.clone(), pin.name.clone(), content_type, bytes)
                    .await;

                if let Ok(bundler_tx_id) = send {
                    let retrieve_bundle = Bundle::retrieve_envelopes(bundler_tx_id.clone()).await;

                    if let Ok(envelopes) = retrieve_bundle {
                        if let Some(item_envelope) = envelopes.envelopes.get(0) {
                            let envelope_hash = item_envelope.hash.clone();

                            let insert_pin_data = create_pin(
                                conn,
                                created_by,
                                &pin.cid,
                                len,
                                &bundler_tx_id,
                                &envelope_hash,
                                pin.name.clone(),
                                &req_id,
                            )
                            .await;

                            println!("Inserted {}", insert_pin_data.is_ok());

                            insert_pin_data.as_ref().err().map(|e| {
                                println!("{:?}", e);
                            });

                            if insert_pin_data.is_ok() {
                                return Ok(PinStatus {
                                    request_id: req_id,
                                    status: Status::Pinned,
                                    created: Default::default(),
                                    pin,
                                    delegates: vec![],
                                    info: Some(self.get_new_status_info(bundler_tx_id)),
                                });
                            }
                        }
                    }
                }
            }
        }

        println!("Failed {:?}", pin);

        Ok(PinStatus {
            request_id: req_id,
            status: Status::Failed,
            created: Default::default(),
            pin,
            delegates: vec![],
            info: None,
        })
    }

    async fn get_pin_by_request_id(&self, request_id: &str) -> actix_web::Result<PinStatus> {
        let conn = self.db_service.get_conn();
        let find = find_pin(conn, request_id.to_string()).await;
        if let Ok(file) = find {
            Ok(PinStatus {
                request_id: file.req_id,
                status: Status::Pinned,
                created: DateTime::parse_from_str(&file.created_at, DATE_FORMAT_MYSQL)
                    .unwrap()
                    .with_timezone(&Utc),
                pin: Pin {
                    cid: file.cid,
                    name: Some(file.name),
                    origins: None,
                    meta: None,
                },
                delegates: vec![],
                info: None,
            })
        } else {
            Err(ErrorNotFound("Pin was not found"))
        }
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
