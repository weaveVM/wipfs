use chrono::{DateTime, NaiveDateTime, Utc};
use enum_as_inner::EnumAsInner;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

/// Response used for listing pin objects matching request
#[derive(Debug, Serialize, Deserialize)]
pub struct PinResults {
    /// The total number of pin objects that exist for passed query filters
    pub count: i32,
    /// An array of PinStatus results
    pub results: Vec<PinStatus>,
}

/// Pin object with status
#[derive(Debug, Serialize, Deserialize)]
pub struct PinStatus {
    /// Globally unique identifier of the pin request;
    /// can be used to check the status of ongoing pinning, or pin removal
    #[serde(rename = "requestid")]
    pub request_id: String,
    /// Current status of the pin
    pub status: Status,
    /// Immutable timestamp indicating when a pin request entered a pinning service
    pub created: DateTime<Utc>,
    /// The pin object
    pub pin: Pin,
    /// List of multiaddrs designated by pinning service that will receive the pin data
    pub delegates: Vec<String>,
    /// Optional info for PinStatus response
    pub info: Option<StatusInfo>,
}

/// Pin object
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pin {
    /// Content Identifier (CID) to be pinned recursively
    pub cid: String,
    /// Optional name for pinned data; can be used for lookups later
    pub name: Option<String>,
    /// Optional list of multiaddrs known to provide the data
    pub origins: Option<Vec<String>>,
    /// Optional metadata for pin object
    pub meta: Option<PinMeta>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreatePin {
    pub pin: Pin,
    pub created_by: i64,
    pub user_token: String,
}

/// Status a pin object can have at a pinning service
#[derive(Debug, Serialize, Deserialize, EnumAsInner)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// Pinning operation is waiting in the queue
    Queued,
    /// Pinning in progress
    Pinning,
    /// Pinned successfully
    Pinned,
    /// Pinning service was unable to finish pinning operation
    Failed,
}

impl FromStr for Status {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "queued" => Ok(Status::Queued),
            "pinning" => Ok(Status::Pinning),
            "pinned" => Ok(Status::Pinned),
            "failed" => Ok(Status::Failed),
            _ => Err(()),
        }
    }
}

/// Optional metadata for pin object
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PinMeta(pub HashMap<String, String>);

impl Default for PinMeta {
    fn default() -> Self {
        PinMeta(HashMap::new())
    }
}

/// Optional info for PinStatus response
#[derive(Debug, Serialize, Deserialize)]
pub struct StatusInfo(pub HashMap<String, String>);

/// Alternative text matching strategy
#[derive(Debug, Serialize, Deserialize, EnumAsInner)]
#[serde(rename_all = "lowercase")]
pub enum TextMatchingStrategy {
    /// Full match, case-sensitive (the implicit default)
    Exact,
    /// Full match, case-insensitive
    Iexact,
    /// Partial match, case-sensitive
    Partial,
    /// Partial match, case-insensitive
    Ipartial,
}

impl FromStr for TextMatchingStrategy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "exact" => Ok(TextMatchingStrategy::Exact),
            "iexact" => Ok(TextMatchingStrategy::Iexact),
            "partial" => Ok(TextMatchingStrategy::Partial),
            "ipartial" => Ok(TextMatchingStrategy::Ipartial),
            _ => Err(()),
        }
    }
}

/// Response for a failed request
#[derive(Debug, Serialize, Deserialize)]
pub struct Failure {
    pub error: ErrorDetails,
}

/// Error details for a failed request
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetails {
    /// Mandatory string identifying the type of error
    pub reason: String,
    /// Optional, longer description of the error
    pub details: Option<String>,
}
