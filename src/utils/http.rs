use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub fn get_internal_call<T: Serialize + DeserializeOwned>(url: String) -> anyhow::Result<T> {
    let api_internal_key = std::env::var("API_INTERNAL_KEY").unwrap_or("".to_string());

    let req = ureq::get(&url).header("X-Load-Auth-Token", api_internal_key);

    let response = req
        .call()
        .map_err(|e| anyhow::anyhow!("HTTP request failed: {}", e))?;

    response
        .into_body()
        .read_json::<T>()
        .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))
}
