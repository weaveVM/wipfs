use crate::structs::{
    CreatePin, Pin, PinMeta, PinResults, PinStatus, Status, TextMatchingStrategy,
};
use actix_web::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetPinsParams {
    pub cid: Option<Vec<String>>,
    pub name: Option<String>,
    pub r#match: Option<TextMatchingStrategy>,
    pub status: Option<Vec<Status>>,
    pub before: Option<DateTime<Utc>>,
    pub after: Option<DateTime<Utc>>,
    pub limit: Option<i32>,
    #[serde(flatten)]
    pub meta: Option<PinMeta>,
    pub created_by: Option<i64>,
}

#[async_trait]
pub trait PinServiceTrait: Send + Sync {
    async fn get_pins(&self, filters: &GetPinsParams) -> Result<PinResults>;
    async fn add_pin(&self, pin: CreatePin) -> Result<PinStatus>;
    async fn get_pin_by_request_id(&self, request_id: &str) -> Result<PinStatus>;
    async fn replace_pin(&self, request_id: &str, pin: Pin) -> Result<PinStatus>;
    async fn delete_pin(&self, request_id: &str) -> Result<()>;
}
