use bundler::utils::core::bundle::Bundle;
use bundler::utils::core::envelope::Envelope;
use bundler::utils::core::tags::Tag;
use bundler::utils::errors::Error;

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
        let mut tags = vec![
            Tag {
                name: "Protocol".to_string(),
                value: "ipfs.rs".to_string(),
            },
            Tag {
                name: "CID".to_string(),
                value: cid,
            },
            Tag {
                name: "Content-Type".to_string(),
                value: content_type,
            },
        ];

        if let Some(pin_name) = name {
            tags.push(Tag {
                name: "Pin-Name".to_string(),
                value: pin_name,
            });
        }

        let envelope = Envelope::new().data(Some(data)).tags(Some(tags));

        let bundle = Bundle::new()
            .private_key(self.private_key.clone())
            .envelopes(vec![envelope])
            .build()?;

        bundle.propagate().await
    }
}
