use crate::db::schema::{AccessKey, FileRecord};
use crate::db::DATE_FORMAT_MYSQL;
use chrono::{DateTime, Utc};
use planetscale_driver::{query, PSConnection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewAccount {
    pub account_name: String,
    pub is_active: bool,
}

#[derive(Serialize, Deserialize)]
pub struct NewAccessKeys {
    pub owner_id: i64,
    pub access_key: String,
    pub is_active: bool,
}

pub async fn create_account(conn: PSConnection, data: NewAccount) -> anyhow::Result<()> {
    let query_str = format!(
        "INSERT INTO accounts(account_name, is_active) VALUES('{}', {})",
        data.account_name, data.is_active
    );

    query(&query_str).execute(&conn).await
}

pub async fn create_access_key(conn: PSConnection, data: NewAccessKeys) -> anyhow::Result<()> {
    let query_str = format!(
        "INSERT INTO access_keys(owner_id, access_key, is_active) VALUES('{}', 'load_acc_{}', {})",
        data.owner_id, data.access_key, data.is_active
    );

    query(&query_str).execute(&conn).await
}

pub async fn find_access_key(
    conn: PSConnection,
    access_key: String,
) -> Result<AccessKey, anyhow::Error> {
    let query_str = format!(
        "SELECT * FROM access_keys WHERE access_key = '{}' LIMIT 1",
        access_key
    );

    query(&query_str).fetch_one(&conn).await
}
