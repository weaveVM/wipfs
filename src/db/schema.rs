use chrono::{DateTime, Utc};
use planetscale_driver::Database;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Database)]
pub struct FileRecord {
    pub id: i64,
    pub created_at: String,
    pub cid: String,
    pub size: i64,
    pub bundle_tx_id: String,
    pub envelope_id: String,
    pub name: String,
    pub req_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Database)]
pub struct AccessKey {
    pub id: i64,
    pub owner_id: i64,
    pub access_key: String,
    pub created_at: String,
    pub is_active: bool,
}
