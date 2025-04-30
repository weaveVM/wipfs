use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub fn get_call<T: Serialize + DeserializeOwned>(url: String) -> anyhow::Result<T> {
    let response = ureq::get(&url)
        .call()
        .map_err(|e| anyhow::anyhow!("HTTP request failed: {}", e))?;

    response
        .into_body()
        .read_json::<T>()
        .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))
}
