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
        cid: String,
        name: Option<String>,
        content_type: String,
        data: Vec<u8>,
    ) -> Result<String, Error> {
        // let mut tags = vec![
        //     Tag {
        //         name: "Protocol".to_string(),
        //         value: "ipfs.rs".to_string(),
        //     },
        //     Tag {
        //         name: "CID".to_string(),
        //         value: cid,
        //     },
        //     Tag {
        //         name: "Content-Type".to_string(),
        //         value: content_type,
        //     },
        // ];

        // if let Some(pin_name) = name {
        //     tags.push(Tag {
        //         name: "Pin-Name".to_string(),
        //         value: pin_name,
        //     });
        // }

        // let envelope = Envelope::new().data(Some(data)).tags(Some(tags));

        // let bundle = Bundle::new()
        //     .private_key(self.private_key.clone())
        //     .envelopes(vec![envelope])
        //     .build()?;

        // // First propagate the bundle to get the transaction ID
        // let tx_id = bundle.propagate().await?;

        // Then send to load0 via HTTP
        match ureq::post("https://load0.network/upload")
            .content_type(&content_type)
            .send(&data[..])
        {
            Ok(response) => match response.into_body().read_to_string() {
                Ok(body) => match serde_json::from_str::<Value>(&body) {
                    Ok(json) => {
                        println!("LOAD0 RESPONSE: {:?}", body);
                        if let Some(hash) = json.get("optimistic_hash").and_then(|h| h.as_str()) {
                            println!("OP HASH: {:?}", hash.to_string());
                            Ok(hash.to_string())
                        } else {
                            Ok(body)
                        }
                    }
                    Err(_) => Ok(body),
                },
                Err(e) => Err(Error::Other(format!("Failed to read response: {}", e))),
            },
            Err(e) => Err(Error::Other(format!("HTTP request failed: {}", e))),
        }
    }
}
