use crate::db::schema::FileRecord;
use crate::db::DATE_FORMAT_MYSQL;
use crate::services::pin_service::GetPinsParams;
use anyhow::Error;
use chrono::{DateTime, NaiveDateTime, Utc};
use planetscale_driver::{query, PSConnection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewFile<'a> {
    pub cid: &'a str,
    pub size: i64,
    pub bundle_tx_id: &'a str,
    pub envelope_id: &'a str,
    pub name: Option<String>,
    pub req_id: &'a str,
}

pub async fn create_pin(
    conn: PSConnection,
    cid: &str,
    size: usize,
    bundle_tx_id: &str,
    envelope_id: &str,
    name: Option<String>,
    req_id: &str,
) -> anyhow::Result<()> {
    let now: DateTime<Utc> = Utc::now();
    let formatted_created_at = now.format(DATE_FORMAT_MYSQL).to_string();

    let query_str = format!(
        "INSERT INTO files(cid, size, bundle_tx_id, envelope_id, name, req_id) VALUES('{}', {}, '{}', '{}', '{}', '{}')",
        cid, size as i64, bundle_tx_id, envelope_id, name.unwrap_or_default(), req_id
    );

    query(&query_str).execute(&conn).await
}

pub async fn find_pins(
    conn: PSConnection,
    params: &GetPinsParams,
) -> Result<Vec<FileRecord>, anyhow::Error> {
    // Start with a base query string. "WHERE 1=1" is a trick to simplify appending "AND" clauses.
    let mut query_str = String::from("SELECT * FROM files WHERE 1=1");

    // If we have a list of CIDs, join them with commas and wrap each in quotes.
    if let Some(cids) = &params.cid {
        let cids_list = cids
            .iter()
            .map(|cid| format!("'{}'", cid))
            .collect::<Vec<String>>()
            .join(", ");
        query_str.push_str(&format!(" AND cid IN ({})", cids_list));
    }

    // If a name filter is provided, add it to the query.
    if let Some(p_name) = &params.name {
        query_str.push_str(&format!(" AND name = '{}'", p_name));
    }

    // If a 'before' timestamp is provided, filter by created_at less than that value.
    if let Some(before) = &params.before {
        query_str.push_str(&format!(" AND created_at < '{}'", before));
    }

    // If an 'after' timestamp is provided, filter by created_at greater than that value.
    if let Some(after) = &params.after {
        query_str.push_str(&format!(" AND created_at > '{}'", after));
    }

    // If a limit is provided, add a LIMIT clause.
    if let Some(limit) = &params.limit {
        query_str.push_str(&format!(" LIMIT {}", limit));
    }

    // Execute the query using your database connection. Adjust the call as necessary for your async runtime.
    query(&query_str).fetch_all(&conn).await
}

pub async fn find_pin(conn: PSConnection, q_cid: String) -> Result<FileRecord, anyhow::Error> {
    let query_str = format!(
        "SELECT * FROM files WHERE cid = '{}' OR req_id = '{}' LIMIT 1",
        q_cid, q_cid
    );

    query(&query_str).fetch_one(&conn).await
}
