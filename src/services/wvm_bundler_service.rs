use bundler::utils::core::bundle::Bundle;
use bundler::utils::core::envelope::Envelope;
use bundler::utils::core::tags::Tag;
use bundler::utils::errors::Error;
use serde_json::Value;

pub struct WvmBundlerService {
    private_key: String,
}

impl WvmBundlerService {
    pub fn new(private_key: String) -> Self {
        Self { private_key }
    }

    pub async fn send(
        &self,
        content_type: String,
        data: Vec<u8>,
    ) -> Result<String, Error> {
        // Send to load0 via HTTP
        let response = ureq::post("https://load0.network/upload")
            .content_type(&content_type)
            .send(&data[..])
            .map_err(|e| Error::Other(format!("HTTP request failed: {}", e)))?;
    
        let body = response.into_body().read_to_string()
            .map_err(|e| Error::Other(format!("Failed to read response: {}", e)))?;
        
        let json_result = serde_json::from_str::<Value>(&body);
        
        match json_result {
            Ok(json) => {
                json.get("optimistic_hash")
                    .and_then(|h| h.as_str())
                    .map(|hash| hash.to_string())
                    .or(Some(body))
                    .ok_or_else(|| Error::Other("Unexpected empty response".to_string()))
            },
            Err(_) => Ok(body),
        }
    }
}
