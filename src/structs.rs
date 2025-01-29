use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
#[derive(Debug, Serialize, Deserialize)]
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

/// Status a pin object can have at a pinning service
#[derive(Debug, Serialize, Deserialize)]
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

/// Optional metadata for pin object
#[derive(Debug, Serialize, Deserialize)]
pub struct PinMeta(pub HashMap<String, String>);

/// Optional info for PinStatus response
#[derive(Debug, Serialize, Deserialize)]
pub struct StatusInfo(pub HashMap<String, String>);

/// Alternative text matching strategy
#[derive(Debug, Serialize, Deserialize)]
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
