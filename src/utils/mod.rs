pub mod http;

use crate::services::pin_service::GetPinsParams;
use std::collections::HashMap;

pub fn parse_query_string(query: &str) -> GetPinsParams {
    let mut params: HashMap<String, Vec<String>> = HashMap::new();

    for (key, value) in form_urlencoded::parse(query.as_bytes()) {
        params
            .entry(key.into_owned())
            .or_insert_with(Vec::new)
            .push(value.into_owned());
    }

    GetPinsParams {
        cid: params.remove("cid").map(|v| v.into_iter().collect()), // Collect all cids
        name: params.remove("name").and_then(|mut v| v.pop()),      // Take first
        r#match: params
            .remove("match")
            .and_then(|mut v| v.pop()?.parse().ok()), // Parse TextMatchingStrategy
        status: params
            .remove("status")
            .map(|v| v.into_iter().filter_map(|s| s.parse().ok()).collect()), // Collect all statuses
        before: params
            .remove("before")
            .and_then(|mut v| v.pop()?.parse().ok()),
        after: params
            .remove("after")
            .and_then(|mut v| v.pop()?.parse().ok()),
        limit: params
            .remove("limit")
            .and_then(|mut v| v.pop()?.parse().ok()),
        meta: None, // Parsing `meta` depends on how `PinMeta` is structured,
        created_by: None,
    }
}
